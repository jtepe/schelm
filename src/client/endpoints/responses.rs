use crate::{client::http, models};

use crate::client::{Client, Error, Result};

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
        input: models::CreateResponseInput,
    ) -> CreateResponseRequestBuilder<'a> {
        CreateResponseRequestBuilder {
            client: self.client,
            body: models::CreateResponseBody {
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
        self.create(model, models::CreateResponseInput::String(text.into()))
    }
}

/// Request builder for `POST /responses`.
#[derive(Debug)]
pub struct CreateResponseRequestBuilder<'a> {
    client: &'a Client,
    body: models::CreateResponseBody,
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

    pub fn tools(mut self, tools: Vec<models::ResponsesToolParam>) -> Self {
        self.body.tools = Some(tools);
        self
    }

    pub fn tool_choice(mut self, tool_choice: models::ToolChoiceParam) -> Self {
        self.body.tool_choice = Some(tool_choice);
        self
    }

    pub fn text(mut self, text: models::TextParam) -> Self {
        self.body.text = Some(text);
        self
    }

    pub fn store(mut self, store: bool) -> Self {
        self.body.store = Some(store);
        self
    }

    pub fn service_tier(mut self, service_tier: models::ServiceTierEnum) -> Self {
        self.body.service_tier = Some(service_tier);
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.body.top_p = Some(top_p);
        self
    }

    pub fn truncation(mut self, truncation: models::TruncationEnum) -> Self {
        self.body.truncation = Some(truncation);
        self
    }

    pub fn reasoning(mut self, reasoning: models::ReasoningParam) -> Self {
        self.body.reasoning = Some(reasoning);
        self
    }

    pub fn previous_response_id(mut self, id: impl Into<String>) -> Self {
        self.body.previous_response_id = Some(id.into());
        self
    }

    /// Sends the request and returns the full response resource.
    pub async fn send(self) -> Result<models::ResponseResource> {
        if self.body.stream == Some(true) {
            return Err(Error::StreamingNotSupported);
        }

        let url = self.client.endpoint_url("responses")?;

        let resp = self.client.http().post(url).json(&self.body).send().await?;

        if !resp.status().is_success() {
            return Err(http::read_error_body(resp).await?);
        }

        Ok(resp.json::<models::ResponseResource>().await?)
    }
}

#[cfg(all(test, feature = "client"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn send_errors_if_stream_is_true() {
        let http = reqwest::Client::builder().build().unwrap();

        // Construct a Client without hitting the network: send() should bail out before using `http`.
        let client = Client {
            base_url: url::Url::parse("https://example.com/v1").unwrap(),
            http,
        };

        let req = Responses::new(&client)
            .create("gpt-test", models::CreateResponseInput::String("hi".into()));

        // We don't expose a stream setter, but we still want to guard against stream=true.
        let req = CreateResponseRequestBuilder {
            client: req.client,
            body: models::CreateResponseBody {
                stream: Some(true),
                ..req.body
            },
        };

        let err = req.send().await.unwrap_err();
        assert!(matches!(err, Error::StreamingNotSupported));
    }
}
