mod common;

use std::pin::Pin;

use futures_core::Stream;
use wiremock::matchers::{body_partial_json, header, method, path};
use wiremock::{Mock, ResponseTemplate};

use schelm_ores::client::{Error, ResponseEventStream, StreamingError};
use schelm_ores::models::StreamingEvent;

/// Helper to pull the next item from a `ResponseEventStream`.
async fn next(
    stream: &mut ResponseEventStream,
) -> Option<schelm_ores::client::Result<StreamingEvent>> {
    std::future::poll_fn(|cx| Pin::new(&mut *stream).poll_next(cx)).await
}

/// Build an SSE body from a sequence of `(event_name, json_data)` pairs,
/// terminated with `data: [DONE]`.
fn sse_body(events: &[(&str, serde_json::Value)]) -> String {
    let mut body = String::new();
    for (event_name, data) in events {
        body.push_str(&format!("event: {event_name}\ndata: {}\n\n", data));
    }
    body.push_str("data: [DONE]\n\n");
    body
}

fn text_delta_event(seq: i32, delta: &str) -> (&'static str, serde_json::Value) {
    (
        "response.output_text.delta",
        serde_json::json!({
            "type": "response.output_text.delta",
            "sequence_number": seq,
            "item_id": "msg_001",
            "output_index": 0,
            "content_index": 0,
            "delta": delta,
            "logprobs": []
        }),
    )
}

// ---------------------------------------------------------------------------
// 1. Happy path — send_stream parses SSE body
// ---------------------------------------------------------------------------

#[tokio::test]
async fn send_stream_happy_path() {
    let server = common::mock_server().await;
    let body = sse_body(&[text_delta_event(0, "Hello"), text_delta_event(1, " world")]);

    Mock::given(method("POST"))
        .and(path("/responses"))
        .and(header("accept", "text/event-stream"))
        .and(body_partial_json(serde_json::json!({ "stream": true })))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(body, "text/event-stream"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let mut stream = client
        .responses()
        .create_text("gpt-test", "Say hello")
        .send_stream()
        .await
        .expect("send_stream should succeed");

    // First event
    let first = next(&mut stream).await.expect("expected first event");
    let event = first.expect("first event should parse");
    match &event {
        StreamingEvent::ResponseOutputTextDelta { delta, .. } => {
            assert_eq!(delta, "Hello");
        }
        other => panic!("expected ResponseOutputTextDelta, got: {other:?}"),
    }

    // Second event
    let second = next(&mut stream).await.expect("expected second event");
    let event = second.expect("second event should parse");
    match &event {
        StreamingEvent::ResponseOutputTextDelta { delta, .. } => {
            assert_eq!(delta, " world");
        }
        other => panic!("expected ResponseOutputTextDelta, got: {other:?}"),
    }

    // Stream should be done
    assert!(next(&mut stream).await.is_none(), "expected stream to end");
}

// ---------------------------------------------------------------------------
// 2. Content-Type validation — rejects non-SSE content type
// ---------------------------------------------------------------------------

#[tokio::test]
async fn send_stream_rejects_wrong_content_type() {
    let server = common::mock_server().await;

    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw("{}", "application/json"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let result = client
        .responses()
        .create_text("gpt-test", "hello")
        .send_stream()
        .await;

    match result {
        Err(Error::Streaming(StreamingError::UnexpectedContentType { got })) => {
            assert_eq!(got.as_deref(), Some("application/json"));
        }
        Err(other) => panic!("expected UnexpectedContentType, got: {other:?}"),
        Ok(_) => panic!("expected error, got Ok"),
    }
}

// ---------------------------------------------------------------------------
// 3. Non-2xx error — send_stream returns HttpStatus error
// ---------------------------------------------------------------------------

#[tokio::test]
async fn send_stream_returns_http_error_on_500() {
    let server = common::mock_server().await;
    let error_body = serde_json::json!({
        "error": {
            "message": "Internal server error",
            "type": "server_error",
            "code": "server_error"
        }
    });

    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(ResponseTemplate::new(500).set_body_json(error_body))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let result = client
        .responses()
        .create_text("gpt-test", "hello")
        .send_stream()
        .await;

    match result {
        Err(Error::HttpStatus { status, body }) => {
            assert_eq!(status.as_u16(), 500);
            assert!(body.contains("server_error"), "body was: {body}");
        }
        Err(other) => panic!("expected HttpStatus error, got: {other:?}"),
        Ok(_) => panic!("expected error, got Ok"),
    }
}
