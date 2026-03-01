//! Responses API endpoint.
//!
//! Provides the [`Responses`] endpoint group and [`ResponseEventStream`] for
//! streaming response events over SSE.

use crate::client::endpoints::sse_core::{SseFrame, SseReader};
use crate::client::error::StreamingError;
use crate::client::{Client, Result, http};
use crate::models::responses;

use futures_core::Stream;

use std::pin::Pin;
use std::task::{Context, Poll};

// ---------------------------------------------------------------------------
// Endpoint group
// ---------------------------------------------------------------------------

/// Responses endpoint group.
#[derive(Clone, Copy, Debug)]
pub struct Responses<'a> {
    client: &'a Client,
}

impl<'a> Responses<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a new response.
    ///
    /// `model` and `input` are required and are provided at construction time.
    pub fn create(
        &self,
        model: impl Into<String>,
        input: responses::CreateResponseInput,
    ) -> CreateResponseRequestBuilder<'a> {
        CreateResponseRequestBuilder {
            client: self.client,
            body: responses::CreateResponseBody {
                model: Some(model.into()),
                input: Some(input),
                previous_response_id: None,
                include: None,
                tools: None,
                tool_choice: None,
                metadata: None,
                text: None,
                temperature: None,
                top_p: None,
                presence_penalty: None,
                frequency_penalty: None,
                parallel_tool_calls: None,
                stream: Some(false),
                stream_options: None,
                background: None,
                max_output_tokens: None,
                max_tool_calls: None,
                reasoning: None,
                safety_identifier: None,
                prompt_cache_key: None,
                truncation: None,
                instructions: None,
                store: Some(false),
                service_tier: None,
                top_logprobs: None,
            },
        }
    }

    /// Convenience helper to create a response from a single user text input.
    pub fn create_text(
        &self,
        model: impl Into<String>,
        text: impl Into<String>,
    ) -> CreateResponseRequestBuilder<'a> {
        self.create(model, responses::CreateResponseInput::String(text.into()))
    }
}

// ---------------------------------------------------------------------------
// Request builder
// ---------------------------------------------------------------------------

/// Request builder for `POST /responses`.
#[derive(Debug)]
pub struct CreateResponseRequestBuilder<'a> {
    client: &'a Client,
    body: responses::CreateResponseBody,
}

impl<'a> CreateResponseRequestBuilder<'a> {
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.body.instructions = Some(instructions.into());
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.body.temperature = Some(temperature);
        self
    }

    pub fn max_output_tokens(mut self, max_output_tokens: i32) -> Self {
        self.body.max_output_tokens = Some(max_output_tokens);
        self
    }

    pub fn tools(mut self, tools: Vec<responses::ResponsesToolParam>) -> Self {
        self.body.tools = Some(tools);
        self
    }

    pub fn tool_choice(mut self, tool_choice: responses::ToolChoiceParam) -> Self {
        self.body.tool_choice = Some(tool_choice);
        self
    }

    pub fn text(mut self, text: responses::TextParam) -> Self {
        self.body.text = Some(text);
        self
    }

    pub fn service_tier(mut self, service_tier: responses::ServiceTierEnum) -> Self {
        self.body.service_tier = Some(service_tier);
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.body.top_p = Some(top_p);
        self
    }

    pub fn truncation(mut self, truncation: responses::TruncationEnum) -> Self {
        self.body.truncation = Some(truncation);
        self
    }

    pub fn reasoning(mut self, reasoning: responses::ReasoningParam) -> Self {
        self.body.reasoning = Some(reasoning);
        self
    }

    pub fn previous_response_id(mut self, id: impl Into<String>) -> Self {
        self.body.previous_response_id = Some(id.into());
        self
    }

    /// Sends the request and returns the full response resource.
    pub async fn send(self) -> Result<responses::ResponseResource> {
        let url = self.client.endpoint_url("responses")?;

        let resp = self.client.http().post(url).json(&self.body).send().await?;

        if !resp.status().is_success() {
            return Err(http::read_error_body(resp).await?);
        }

        Ok(resp.json::<responses::ResponseResource>().await?)
    }

    /// Sends the request with streaming enabled and returns a stream of events.
    ///
    /// This force-sets `stream=true` on the request body. The returned
    /// [`ResponseEventStream`] yields `Result<StreamingEvent>` items decoded
    /// from the SSE response.
    pub async fn send_stream(mut self) -> Result<ResponseEventStream> {
        self.body.stream = Some(true);

        let url = self.client.endpoint_url("responses")?;

        let resp = self
            .client
            .http()
            .post(url)
            .header(reqwest::header::ACCEPT, "text/event-stream")
            .json(&self.body)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(http::read_error_body(resp).await?);
        }

        ResponseEventStream::from_response(resp)
    }
}

// ---------------------------------------------------------------------------
// Responses-specific SSE stream
// ---------------------------------------------------------------------------

/// A stream of [`StreamingEvent`](responses::StreamingEvent) items decoded from
/// an SSE byte stream, specific to the Responses API.
///
/// Wraps the shared [`SseReader`] and applies Responses-specific JSON decoding
/// (type injection, mismatch detection, etc.).
pub struct ResponseEventStream {
    reader: SseReader,
}

impl ResponseEventStream {
    /// Creates a new `ResponseEventStream` from a reqwest response.
    ///
    /// Validates that the content-type is `text/event-stream` before constructing.
    pub(crate) fn from_response(resp: reqwest::Response) -> Result<Self> {
        Ok(Self {
            reader: SseReader::from_response(resp)?,
        })
    }

    /// Creates a `ResponseEventStream` from any byte chunk stream.
    ///
    /// Used internally for testing without reqwest.
    #[cfg(test)]
    pub(crate) fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = std::result::Result<bytes::Bytes, reqwest::Error>> + Send + 'static,
    {
        Self {
            reader: SseReader::from_stream(stream),
        }
    }
}

/// Decode a single SSE frame into a Responses API `StreamingEvent`.
fn decode_frame(frame: SseFrame) -> Result<Option<responses::StreamingEvent>> {
    let SseFrame { event, data } = frame;

    if data.is_empty() {
        return Ok(None);
    }

    if data == "[DONE]" {
        return Ok(None);
    }

    // Try direct deserialization first
    match serde_json::from_str::<responses::StreamingEvent>(&data) {
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
                            map.insert("type".to_owned(), serde_json::Value::String(event_name));
                            let injected = serde_json::Value::Object(map);
                            return serde_json::from_value::<responses::StreamingEvent>(injected)
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
    type Item = Result<responses::StreamingEvent>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            match Pin::new(&mut this.reader).poll_next(cx) {
                Poll::Ready(Some(Ok(frame))) => {
                    match decode_frame(frame) {
                        Ok(Some(event)) => return Poll::Ready(Some(Ok(event))),
                        Ok(None) => continue, // skip empty/done frames
                        Err(e) => return Poll::Ready(Some(Err(e))),
                    }
                }
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e))),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::endpoints::sse_core::MAX_EVENT_BYTES;
    use bytes::Bytes;
    use std::collections::VecDeque;

    /// A simple in-memory stream of byte chunks for testing.
    struct TestStream {
        chunks: VecDeque<Bytes>,
    }

    impl TestStream {
        fn new(chunks: Vec<Bytes>) -> Self {
            Self {
                chunks: chunks.into(),
            }
        }
    }

    impl Stream for TestStream {
        type Item = std::result::Result<Bytes, reqwest::Error>;

        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            match self.get_mut().chunks.pop_front() {
                Some(chunk) => Poll::Ready(Some(Ok(chunk))),
                None => Poll::Ready(None),
            }
        }
    }

    /// Helper to pull the next item from a `ResponseEventStream`.
    async fn next(stream: &mut ResponseEventStream) -> Option<Result<responses::StreamingEvent>> {
        std::future::poll_fn(|cx| Pin::new(&mut *stream).poll_next(cx)).await
    }

    /// Helper to collect all items from a `ResponseEventStream`.
    async fn collect_all(
        stream: &mut ResponseEventStream,
    ) -> Vec<Result<responses::StreamingEvent>> {
        let mut events = Vec::new();
        while let Some(item) = next(stream).await {
            events.push(item);
        }
        events
    }

    /// A minimal `response.output_text.delta` JSON payload.
    fn text_delta_json(seq: i32, delta: &str) -> String {
        serde_json::json!({
            "type": "response.output_text.delta",
            "sequence_number": seq,
            "item_id": "msg_001",
            "output_index": 0,
            "content_index": 0,
            "delta": delta,
            "logprobs": []
        })
        .to_string()
    }

    /// Wraps JSON data in an SSE frame with optional `event:` line.
    fn sse_frame(event: Option<&str>, data: &str) -> String {
        let mut frame = String::new();
        if let Some(e) = event {
            frame.push_str(&format!("event: {e}\n"));
        }
        frame.push_str(&format!("data: {data}\n\n"));
        frame
    }

    // -----------------------------------------------------------------------
    // 1. Parses multiple events
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn parses_multiple_events() {
        let body = format!(
            "{}{}{}",
            sse_frame(
                Some("response.output_text.delta"),
                &text_delta_json(0, "Hello"),
            ),
            sse_frame(
                Some("response.output_text.delta"),
                &text_delta_json(1, " world"),
            ),
            "data: [DONE]\n\n",
        );

        let stream = TestStream::new(vec![Bytes::from(body)]);
        let mut event_stream = ResponseEventStream::from_stream(stream);
        let events = collect_all(&mut event_stream).await;

        assert_eq!(events.len(), 2);
        for event in &events {
            assert!(event.is_ok(), "expected Ok, got: {event:?}");
        }

        match events[0].as_ref().unwrap() {
            responses::StreamingEvent::ResponseOutputTextDelta { delta, .. } => {
                assert_eq!(delta, "Hello");
            }
            other => panic!("expected ResponseOutputTextDelta, got: {other:?}"),
        }
        match events[1].as_ref().unwrap() {
            responses::StreamingEvent::ResponseOutputTextDelta { delta, .. } => {
                assert_eq!(delta, " world");
            }
            other => panic!("expected ResponseOutputTextDelta, got: {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // 2. Tolerant injection — SSE event name injected as "type" when missing
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn tolerant_injection_adds_type_from_event_name() {
        // JSON is missing the "type" field; SSE `event:` line provides it.
        let data = serde_json::json!({
            "sequence_number": 0,
            "item_id": "msg_001",
            "output_index": 0,
            "content_index": 0,
            "delta": "injected",
            "logprobs": []
        })
        .to_string();

        let body = format!(
            "{}{}",
            sse_frame(Some("response.output_text.delta"), &data),
            "data: [DONE]\n\n",
        );

        let stream = TestStream::new(vec![Bytes::from(body)]);
        let mut event_stream = ResponseEventStream::from_stream(stream);
        let events = collect_all(&mut event_stream).await;

        assert_eq!(events.len(), 1);
        match events[0].as_ref().unwrap() {
            responses::StreamingEvent::ResponseOutputTextDelta { delta, .. } => {
                assert_eq!(delta, "injected");
            }
            other => panic!("expected ResponseOutputTextDelta, got: {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // 3. Mismatch detection — SSE event name disagrees with JSON type
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn mismatch_detection_errors() {
        // SSE says "response.completed" but JSON says "response.output_text.delta"
        let data = text_delta_json(0, "mismatch");
        let body = format!(
            "{}{}",
            sse_frame(Some("response.completed"), &data),
            "data: [DONE]\n\n",
        );

        let stream = TestStream::new(vec![Bytes::from(body)]);
        let mut event_stream = ResponseEventStream::from_stream(stream);

        let event = next(&mut event_stream).await;
        assert!(event.is_some());
        let err = event.unwrap().unwrap_err();
        match err {
            crate::client::Error::Streaming(StreamingError::TypeMismatch { event, ty }) => {
                assert_eq!(event, "response.completed");
                assert_eq!(ty, "response.output_text.delta");
            }
            other => panic!("expected TypeMismatch, got: {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // 4. Chunk-boundary robustness — event split across byte chunks
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn chunk_boundary_robustness() {
        let full = format!(
            "{}{}",
            sse_frame(
                Some("response.output_text.delta"),
                &text_delta_json(0, "split"),
            ),
            "data: [DONE]\n\n",
        );

        // Split roughly in the middle of the frame
        let mid = full.len() / 2;
        let chunk1 = Bytes::from(full[..mid].to_owned());
        let chunk2 = Bytes::from(full[mid..].to_owned());

        let stream = TestStream::new(vec![chunk1, chunk2]);
        let mut event_stream = ResponseEventStream::from_stream(stream);

        let events = collect_all(&mut event_stream).await;
        assert_eq!(events.len(), 1);
        assert!(events[0].is_ok());
        match events[0].as_ref().unwrap() {
            responses::StreamingEvent::ResponseOutputTextDelta { delta, .. } => {
                assert_eq!(delta, "split");
            }
            other => panic!("expected ResponseOutputTextDelta, got: {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // 5. [DONE] termination — stream ends cleanly
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn done_marker_terminates_stream() {
        let body = format!(
            "{}{}",
            sse_frame(
                Some("response.output_text.delta"),
                &text_delta_json(0, "before done"),
            ),
            "data: [DONE]\n\n",
        );

        let stream = TestStream::new(vec![Bytes::from(body)]);
        let mut event_stream = ResponseEventStream::from_stream(stream);

        // First item should be the delta
        let first = next(&mut event_stream).await;
        assert!(first.is_some());
        assert!(first.unwrap().is_ok());

        // Stream should be terminated (no more items)
        let second = next(&mut event_stream).await;
        assert!(second.is_none(), "expected None after [DONE]");
    }

    // -----------------------------------------------------------------------
    // 6. Event-too-large — exceeds MAX_EVENT_BYTES
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn event_too_large_errors() {
        // Send a chunk larger than MAX_EVENT_BYTES without a frame delimiter
        // so the buffer grows past the limit.
        let oversized = vec![b'x'; MAX_EVENT_BYTES + 1];
        let stream = TestStream::new(vec![Bytes::from(oversized)]);
        let mut event_stream = ResponseEventStream::from_stream(stream);

        let event = next(&mut event_stream).await;
        assert!(event.is_some());
        let err = event.unwrap().unwrap_err();
        match err {
            crate::client::Error::Streaming(StreamingError::EventTooLarge { limit_bytes }) => {
                assert_eq!(limit_bytes, MAX_EVENT_BYTES);
            }
            other => panic!("expected EventTooLarge, got: {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // 7. Unknown event type — does not kill the stream
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn unsupported_event_type_does_not_kill_stream() {
        let unknown_json = serde_json::json!({
            "type": "response.new_unknown.delta",
            "sequence_number": 0,
            "content": "thinking"
        })
        .to_string();

        let body = format!(
            "{}{}{}",
            sse_frame(Some("response.new_unknown.delta"), &unknown_json),
            sse_frame(
                Some("response.output_text.delta"),
                &text_delta_json(1, "Hello"),
            ),
            "data: [DONE]\n\n",
        );

        let stream = TestStream::new(vec![Bytes::from(body)]);
        let mut event_stream = ResponseEventStream::from_stream(stream);
        let events = collect_all(&mut event_stream).await;

        assert_eq!(events.len(), 2);

        // First event should be Unknown
        match events[0].as_ref().unwrap() {
            responses::StreamingEvent::Unknown(u) => {
                assert_eq!(u.event_type, "response.new_unknown.delta");
                assert_eq!(
                    u.payload.get("content").unwrap(),
                    &serde_json::json!("thinking")
                );
            }
            other => panic!("expected Unknown, got: {other:?}"),
        }

        // Second event should be the known text delta
        match events[1].as_ref().unwrap() {
            responses::StreamingEvent::ResponseOutputTextDelta { delta, .. } => {
                assert_eq!(delta, "Hello");
            }
            other => panic!("expected ResponseOutputTextDelta, got: {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // 8. Unknown event with type injection from SSE header
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn unknown_event_with_type_injection() {
        // JSON body has no "type" field; SSE event: header provides it
        let data = r#"{"sequence_number":0,"data":"something"}"#;
        let body = format!(
            "{}{}",
            sse_frame(Some("response.new_feature"), data),
            "data: [DONE]\n\n",
        );

        let stream = TestStream::new(vec![Bytes::from(body)]);
        let mut event_stream = ResponseEventStream::from_stream(stream);
        let events = collect_all(&mut event_stream).await;

        assert_eq!(events.len(), 1);
        match events[0].as_ref().unwrap() {
            responses::StreamingEvent::Unknown(u) => {
                assert_eq!(u.event_type, "response.new_feature");
                assert_eq!(
                    u.payload.get("data").unwrap(),
                    &serde_json::json!("something")
                );
            }
            other => panic!("expected Unknown, got: {other:?}"),
        }
    }
}
