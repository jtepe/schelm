pub mod common;

use schelm_http::client::{Client, ClientBuilder, MessagesApi, ResponsesApi};
use wiremock::matchers::{bearer_token, method, path};
use wiremock::{Mock, ResponseTemplate};

// ---------------------------------------------------------------------------
// Auth header behaviour — verify correct auth scheme per API marker
// ---------------------------------------------------------------------------

#[tokio::test]
async fn responses_builder_sends_bearer_auth() {
    let server = common::mock_server().await;

    Mock::given(method("POST"))
        .and(path("/responses"))
        .and(bearer_token("resp-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(common::success_response_body()))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = url::Url::parse(&server.uri()).unwrap();
    let client = ClientBuilder::responses("resp-key", base_url)
        .build()
        .unwrap();

    let resp = client
        .responses()
        .create_text("gpt-test", "hello")
        .send()
        .await
        .expect("bearer auth should match");

    assert_eq!(resp.id, "resp_test_123");
}

// ---------------------------------------------------------------------------
// Type-safety — typed constructors return correctly parameterized builders
// ---------------------------------------------------------------------------

#[test]
fn responses_constructor_returns_responses_builder() {
    let base_url = url::Url::parse("https://example.com/v1").unwrap();
    // This line asserts the return type at compile time.
    let _builder: ClientBuilder<ResponsesApi> = ClientBuilder::responses("key", base_url);
}

#[test]
fn messages_constructor_returns_messages_builder() {
    let base_url = url::Url::parse("https://example.com/v1").unwrap();
    // This line asserts the return type at compile time.
    let _builder: ClientBuilder<MessagesApi> = ClientBuilder::messages("key", base_url);
}

#[test]
fn builder_preserves_marker_through_build() {
    let base_url = url::Url::parse("https://example.com/v1").unwrap();
    let _client: Client<ResponsesApi> = ClientBuilder::responses("key", base_url).build().unwrap();
}
