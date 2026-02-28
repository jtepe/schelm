pub mod common;

use wiremock::matchers::{bearer_token, body_partial_json, header, method, path};
use wiremock::{Mock, ResponseTemplate};

// ---------------------------------------------------------------------------
// Happy-path tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_response_returns_parsed_resource() {
    let server = common::mock_server().await;

    Mock::given(method("POST"))
        .and(path("/responses"))
        .and(header("content-type", "application/json"))
        .and(bearer_token("test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(common::success_response_body()))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let resp = client
        .responses()
        .create_text("gpt-test", "Say hello")
        .send()
        .await
        .expect("request should succeed");

    assert_eq!(resp.id, "resp_test_123");
    assert_eq!(resp.object, "response");
    assert_eq!(resp.status, "completed");
    assert_eq!(resp.model, "gpt-test");
    assert!(resp.usage.is_some());

    let usage = resp.usage.unwrap();
    assert_eq!(usage.input_tokens, 10);
    assert_eq!(usage.output_tokens, 5);
    assert_eq!(usage.total_tokens, 15);
}

#[tokio::test]
async fn create_response_sends_correct_request_body() {
    let server = common::mock_server().await;

    Mock::given(method("POST"))
        .and(path("/responses"))
        .and(body_partial_json(serde_json::json!({
            "model": "gpt-test",
            "input": "Say hello",
            "stream": false,
            "store": false
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(common::success_response_body()))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    client
        .responses()
        .create_text("gpt-test", "Say hello")
        .send()
        .await
        .expect("request should succeed");
}

#[tokio::test]
async fn create_response_with_optional_params() {
    let server = common::mock_server().await;

    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(common::success_response_body()))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let resp = client
        .responses()
        .create_text("gpt-test", "Say hello")
        .temperature(0.5)
        .max_output_tokens(100)
        .instructions("Be concise")
        .top_p(0.9)
        .send()
        .await
        .expect("request should succeed");

    assert_eq!(resp.id, "resp_test_123");
}

// ---------------------------------------------------------------------------
// Error-path tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn returns_http_status_error_on_401() {
    let server = common::mock_server().await;
    let error_body = serde_json::json!({
        "error": {
            "message": "Invalid API key",
            "type": "authentication_error",
            "code": "invalid_api_key"
        }
    });

    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_body))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let err = client
        .responses()
        .create_text("gpt-test", "hello")
        .send()
        .await
        .expect_err("should return an error for 401");

    match err {
        schelm_ores::client::Error::HttpStatus { status, body } => {
            assert_eq!(status.as_u16(), 401);
            assert!(body.contains("invalid_api_key"), "body was: {body}");
        }
        other => panic!("expected HttpStatus error, got: {other:?}"),
    }
}

#[tokio::test]
async fn returns_http_status_error_on_400() {
    let server = common::mock_server().await;
    let error_body = serde_json::json!({
        "error": {
            "message": "Invalid request: model is required",
            "type": "invalid_request_error",
            "code": "invalid_request"
        }
    });

    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(ResponseTemplate::new(400).set_body_json(error_body))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let err = client
        .responses()
        .create_text("gpt-test", "hello")
        .send()
        .await
        .expect_err("should return an error for 400");

    match err {
        schelm_ores::client::Error::HttpStatus { status, body } => {
            assert_eq!(status.as_u16(), 400);
            assert!(body.contains("invalid_request"), "body was: {body}");
        }
        other => panic!("expected HttpStatus error, got: {other:?}"),
    }
}

#[tokio::test]
async fn returns_http_status_error_on_500() {
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
    let err = client
        .responses()
        .create_text("gpt-test", "hello")
        .send()
        .await
        .expect_err("should return an error for 500");

    match err {
        schelm_ores::client::Error::HttpStatus { status, body } => {
            assert_eq!(status.as_u16(), 500);
            assert!(body.contains("server_error"), "body was: {body}");
        }
        other => panic!("expected HttpStatus error, got: {other:?}"),
    }
}

#[tokio::test]
async fn returns_http_status_error_on_429_rate_limit() {
    let server = common::mock_server().await;
    let error_body = serde_json::json!({
        "error": {
            "message": "Rate limit exceeded",
            "type": "rate_limit_error",
            "code": "rate_limit_exceeded"
        }
    });

    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(ResponseTemplate::new(429).set_body_json(error_body))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let err = client
        .responses()
        .create_text("gpt-test", "hello")
        .send()
        .await
        .expect_err("should return an error for 429");

    match err {
        schelm_ores::client::Error::HttpStatus { status, body } => {
            assert_eq!(status.as_u16(), 429);
            assert!(body.contains("rate_limit_exceeded"), "body was: {body}");
        }
        other => panic!("expected HttpStatus error, got: {other:?}"),
    }
}

#[tokio::test]
async fn returns_error_on_invalid_json_response() {
    let server = common::mock_server().await;

    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(ResponseTemplate::new(200).set_body_string("this is not json"))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let err = client
        .responses()
        .create_text("gpt-test", "hello")
        .send()
        .await
        .expect_err("should return an error for invalid JSON");

    // reqwest surfaces deserialization failures as its own error type
    assert!(
        matches!(err, schelm_ores::client::Error::Reqwest(_)),
        "expected Reqwest error, got: {err:?}"
    );
}

#[tokio::test]
async fn returns_http_status_error_with_empty_body() {
    let server = common::mock_server().await;

    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(ResponseTemplate::new(502))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let err = client
        .responses()
        .create_text("gpt-test", "hello")
        .send()
        .await
        .expect_err("should return an error for 502");

    match err {
        schelm_ores::client::Error::HttpStatus { status, body } => {
            assert_eq!(status.as_u16(), 502);
            assert!(body.is_empty(), "expected empty body, got: {body}");
        }
        other => panic!("expected HttpStatus error, got: {other:?}"),
    }
}

#[tokio::test]
async fn unmatched_request_returns_error() {
    // When wiremock has no matching mock, it responds with 404.
    // The client should surface that as an HttpStatus error.
    let server = common::mock_server().await;
    // Intentionally mount no mocks.

    let client = common::test_client(&server);
    let err = client
        .responses()
        .create_text("gpt-test", "hello")
        .send()
        .await
        .expect_err("should return an error when no mock matches");

    match err {
        schelm_ores::client::Error::HttpStatus { status, .. } => {
            assert_eq!(status.as_u16(), 404);
        }
        other => panic!("expected HttpStatus error, got: {other:?}"),
    }
}
