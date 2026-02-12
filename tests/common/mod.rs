use wiremock::MockServer;

/// Starts a dedicated [`MockServer`] that is **not** drawn from the global pool.
///
/// Using [`MockServer::builder().start()`] instead of [`MockServer::start()`]
/// ensures each test gets its own isolated server instance, avoiding shared
/// state that can cause leaky / flaky tests.
pub async fn mock_server() -> MockServer {
    MockServer::builder().start().await
}

/// Builds an [`ores::client::Client`] whose base URL points at the given mock
/// server so that all HTTP traffic stays local.
pub fn test_client(server: &MockServer) -> ores::client::Client {
    let base_url = url::Url::parse(&server.uri()).expect("mock server URI should be a valid URL");
    ores::client::Client::builder("test-api-key", base_url)
        .build()
        .expect("client builder should not fail with valid inputs")
}

/// Returns a minimal, valid JSON body representing a successful
/// `ResponseResource` from the API.
///
/// The caller can layer additional fields on top via `serde_json::Value`
/// mutation if a test needs custom output items, errors, etc.
pub fn success_response_body() -> serde_json::Value {
    serde_json::json!({
        "id": "resp_test_123",
        "object": "response",
        "created_at": 1700000000,
        "completed_at": 1700000001,
        "status": "completed",
        "incomplete_details": null,
        "model": "gpt-test",
        "previous_response_id": null,
        "instructions": null,
        "output": [
            {
                "type": "message",
                "id": "msg_test_001",
                "status": "completed",
                "role": "assistant",
                "content": [
                    {
                        "type": "output_text",
                        "text": "Hello from the mock!",
                        "annotations": [],
                        "logprobs": []
                    }
                ]
            }
        ],
        "error": null,
        "tools": [],
        "tool_choice": null,
        "truncation": "disabled",
        "parallel_tool_calls": false,
        "text": {
            "format": { "type": "text" }
        },
        "top_p": 1.0,
        "presence_penalty": 0.0,
        "frequency_penalty": 0.0,
        "top_logprobs": 0,
        "temperature": 1.0,
        "reasoning": null,
        "usage": {
            "input_tokens": 10,
            "output_tokens": 5,
            "total_tokens": 15,
            "input_tokens_details": { "cached_tokens": 0 },
            "output_tokens_details": { "reasoning_tokens": 0 }
        },
        "max_output_tokens": null,
        "max_tool_calls": null,
        "store": false,
        "background": false,
        "service_tier": "default",
        "metadata": {},
        "safety_identifier": null,
        "prompt_cache_key": null
    })
}
