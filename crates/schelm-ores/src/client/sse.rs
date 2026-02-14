//! SSE (Server-Sent Events) decoder for streaming responses.
//!
//! Consumes a byte stream and yields `Result<StreamingEvent>` items.

use crate::client::Result;
use crate::client::error::StreamingError;
use crate::models::StreamingEvent;

use bytes::Bytes;
use futures_core::Stream;

use std::pin::Pin;
use std::task::{Context, Poll};

/// Maximum size of a single SSE event payload in bytes (1 MiB).
const MAX_EVENT_BYTES: usize = 1024 * 1024;

/// A stream of `StreamingEvent` items decoded from an SSE byte stream.
///
/// Created via [`ResponseEventStream::new`]. Implements [`futures_core::Stream`].
pub struct ResponseEventStream {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    buf: Vec<u8>,
    done: bool,
}

impl ResponseEventStream {
    /// Creates a new `ResponseEventStream` from a reqwest response.
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

    /// Creates a `ResponseEventStream` from any byte chunk stream.
    ///
    /// Used internally for testing without reqwest.
    #[cfg(test)]
    pub(crate) fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
    {
        Self {
            inner: Box::pin(stream),
            buf: Vec::new(),
            done: false,
        }
    }
}

/// Validates the `Content-Type` header of a response.
fn validate_content_type(resp: &reqwest::Response) -> Result<()> {
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

/// Represents a parsed SSE frame before JSON decoding.
struct SseFrame {
    event: Option<String>,
    data: String,
}

/// Attempt to extract the next complete SSE frame from the buffer.
///
/// Returns `Some((frame, consumed_bytes))` if a complete frame was found.
fn extract_frame(buf: &[u8]) -> Option<(SseFrame, usize)> {
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

/// Decode a single SSE frame into a `StreamingEvent`.
fn decode_frame(frame: SseFrame) -> Result<Option<StreamingEvent>> {
    let SseFrame { event, data } = frame;

    if data.is_empty() {
        // Empty data frame (e.g. keepalive) — skip
        return Ok(None);
    }

    // Terminal marker
    if data == "[DONE]" {
        return Ok(None);
    }

    // Try direct deserialization first
    match serde_json::from_str::<StreamingEvent>(&data) {
        Ok(streaming_event) => {
            // If both SSE event: and JSON type exist, verify they agree
            if let Some(event_name) = event {
                let json_type = extract_json_type(&data);
                if let Some(ty) = json_type
                    && ty != event_name
                {
                    return Err(StreamingError::TypeMismatch {
                        event: event_name,
                        ty,
                    }
                    .into());
                }
            }
            Ok(Some(streaming_event))
        }
        Err(first_err) => {
            // If we have an SSE event name, try injecting it as "type"
            if let Some(event_name) = event {
                match serde_json::from_str::<serde_json::Value>(&data) {
                    Ok(serde_json::Value::Object(mut map)) => {
                        if !map.contains_key("type") {
                            map.insert(
                                "type".to_owned(),
                                serde_json::Value::String(event_name),
                            );
                            let injected = serde_json::Value::Object(map);
                            return serde_json::from_value::<StreamingEvent>(injected)
                                .map(Some)
                                .map_err(|e| {
                                    StreamingError::Json {
                                        source: e,
                                        payload: data,
                                    }
                                    .into()
                                });
                        }
                        // Has "type" but disagrees with event name — already checked above in Ok path
                        // Reaching here means direct deser failed but JSON has "type".
                        // Let's check for mismatch:
                        if let Some(serde_json::Value::String(ty)) = map.get("type")
                            && ty != &event_name
                        {
                            return Err(StreamingError::TypeMismatch {
                                event: event_name,
                                ty: ty.clone(),
                            }
                            .into());
                        }
                        // Type matches but deser still failed — report original error
                        Err(StreamingError::Json {
                            source: first_err,
                            payload: data,
                        }
                        .into())
                    }
                    _ => Err(StreamingError::Json {
                        source: first_err,
                        payload: data,
                    }
                    .into()),
                }
            } else {
                Err(StreamingError::Json {
                    source: first_err,
                    payload: data,
                }
                .into())
            }
        }
    }
}

/// Extract the "type" field from a JSON string without full deserialization.
fn extract_json_type(json: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(json)
        .ok()
        .and_then(|v| v.get("type").and_then(|t| t.as_str()).map(|s| s.to_owned()))
}

impl Stream for ResponseEventStream {
    type Item = Result<StreamingEvent>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if this.done {
            return Poll::Ready(None);
        }

        loop {
            // Try to extract a frame from the buffer first
            if let Some((frame, consumed)) = extract_frame(&this.buf) {
                this.buf.drain(..consumed);

                // Skip empty keepalive frames
                if frame.data.is_empty() && frame.event.is_none() {
                    continue;
                }

                // Check for [DONE] termination
                if frame.data == "[DONE]" {
                    this.done = true;
                    return Poll::Ready(None);
                }

                match decode_frame(frame) {
                    Ok(Some(event)) => return Poll::Ready(Some(Ok(event))),
                    Ok(None) => continue, // skip empty/done frames
                    Err(e) => {
                        this.done = true;
                        return Poll::Ready(Some(Err(e)));
                    }
                }
            }

            // Need more data — poll the inner stream
            match Pin::new(&mut this.inner).poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    this.buf.extend_from_slice(&chunk);

                    // Safety limit check
                    if this.buf.len() > MAX_EVENT_BYTES {
                        this.done = true;
                        return Poll::Ready(Some(Err(StreamingError::EventTooLarge {
                            limit_bytes: MAX_EVENT_BYTES,
                        }
                        .into())));
                    }

                    // Loop back to try frame extraction
                }
                Poll::Ready(Some(Err(e))) => {
                    this.done = true;
                    return Poll::Ready(Some(Err(e.into())));
                }
                Poll::Ready(None) => {
                    // Stream ended — check if there's remaining data without termination
                    this.done = true;
                    return Poll::Ready(None);
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
