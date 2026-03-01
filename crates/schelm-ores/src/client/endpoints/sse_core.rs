//! Shared SSE (Server-Sent Events) transport infrastructure.
//!
//! Provides frame extraction, content-type validation, buffer handling, and
//! max-size enforcement for any endpoint that consumes an SSE byte stream.
//! Endpoint-specific decoding (e.g. mapping frames to `StreamingEvent`) lives
//! next to each endpoint implementation.

use crate::client::Result;
use crate::client::error::StreamingError;

use bytes::Bytes;
use futures_core::Stream;

use std::pin::Pin;
use std::task::{Context, Poll};

/// Maximum size of a single SSE event payload in bytes (1 MiB).
pub(crate) const MAX_EVENT_BYTES: usize = 1024 * 1024;

/// A parsed SSE frame before any JSON decoding.
pub(crate) struct SseFrame {
    pub event: Option<String>,
    pub data: String,
}

/// Validates the `Content-Type` header of a response.
pub(crate) fn validate_content_type(resp: &reqwest::Response) -> Result<()> {
    let ct = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    match &ct {
        Some(v) if v.starts_with("text/event-stream") => Ok(()),
        _ => Err(StreamingError::UnexpectedContentType { got: ct }.into()),
    }
}

/// Attempt to extract the next complete SSE frame from the buffer.
///
/// Returns `Some((frame, consumed_bytes))` if a complete frame was found.
pub(crate) fn extract_frame(buf: &[u8]) -> Option<(SseFrame, usize)> {
    // Look for a blank-line delimiter: \n\n or \r\n\r\n
    let s = std::str::from_utf8(buf).ok()?;

    let (frame_text, consumed) = if let Some(pos) = s.find("\r\n\r\n") {
        (&s[..pos], pos + 4)
    } else if let Some(pos) = s.find("\n\n") {
        (&s[..pos], pos + 2)
    } else {
        return None;
    };

    let mut event_name: Option<String> = None;
    let mut data_lines: Vec<&str> = Vec::new();

    for line in frame_text.lines() {
        if line.starts_with(':') {
            // Comment line — ignore
            continue;
        }

        if let Some(value) = line.strip_prefix("event:") {
            event_name = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("data:") {
            data_lines.push(value.trim_start_matches(' '));
        } else if line.starts_with("id:") || line.starts_with("retry:") {
            // Ignored for now
        }
        // Lines without a colon are ignored per SSE spec
    }

    if data_lines.is_empty() && event_name.is_none() {
        // Empty frame (e.g. keepalive) — skip it
        return Some((
            SseFrame {
                event: None,
                data: String::new(),
            },
            consumed,
        ));
    }

    let data = data_lines.join("\n");

    Some((
        SseFrame {
            event: event_name,
            data,
        },
        consumed,
    ))
}

/// A raw SSE byte-stream reader that buffers incoming chunks and yields
/// parsed [`SseFrame`]s. Handles max-size enforcement and `[DONE]` termination.
///
/// Endpoint-specific stream types wrap this to decode frames into their own
/// event types.
pub(crate) struct SseReader {
    inner: Pin<Box<dyn Stream<Item = std::result::Result<Bytes, reqwest::Error>> + Send>>,
    buf: Vec<u8>,
    done: bool,
}

impl SseReader {
    /// Creates a new `SseReader` from a reqwest response.
    ///
    /// Validates that the content-type is `text/event-stream` before constructing.
    pub(crate) fn from_response(resp: reqwest::Response) -> Result<Self> {
        validate_content_type(&resp)?;
        Ok(Self {
            inner: Box::pin(resp.bytes_stream()),
            buf: Vec::new(),
            done: false,
        })
    }

    /// Creates an `SseReader` from any byte chunk stream.
    ///
    /// Used internally for testing without reqwest.
    #[cfg(test)]
    pub(crate) fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = std::result::Result<Bytes, reqwest::Error>> + Send + 'static,
    {
        Self {
            inner: Box::pin(stream),
            buf: Vec::new(),
            done: false,
        }
    }

    /// Polls for the next SSE frame from the underlying byte stream.
    ///
    /// Returns `Poll::Ready(Some(Ok(frame)))` when a complete frame is available,
    /// `Poll::Ready(None)` on `[DONE]` or stream end, or an error on failures.
    pub(crate) fn poll_next_frame(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<SseFrame>>> {
        if self.done {
            return Poll::Ready(None);
        }

        loop {
            // Try to extract a frame from the buffer first
            if let Some((frame, consumed)) = extract_frame(&self.buf) {
                self.buf.drain(..consumed);

                // Skip empty keepalive frames
                if frame.data.is_empty() && frame.event.is_none() {
                    continue;
                }

                // Check for [DONE] termination
                if frame.data == "[DONE]" {
                    self.done = true;
                    return Poll::Ready(None);
                }

                return Poll::Ready(Some(Ok(frame)));
            }

            // Need more data — poll the inner stream
            match self.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    self.buf.extend_from_slice(&chunk);

                    // Safety limit check
                    if self.buf.len() > MAX_EVENT_BYTES {
                        self.done = true;
                        return Poll::Ready(Some(Err(StreamingError::EventTooLarge {
                            limit_bytes: MAX_EVENT_BYTES,
                        }
                        .into())));
                    }

                    // Loop back to try frame extraction
                }
                Poll::Ready(Some(Err(e))) => {
                    self.done = true;
                    return Poll::Ready(Some(Err(e.into())));
                }
                Poll::Ready(None) => {
                    // Stream ended — check if there's remaining data without termination
                    self.done = true;
                    return Poll::Ready(None);
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
