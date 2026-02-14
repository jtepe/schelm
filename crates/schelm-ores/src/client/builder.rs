use crate::client::{Client, Error, Result, http};

/// Builder for [`Client`].
///
/// Required fields are provided via [`ClientBuilder::new`].
#[derive(Debug)]
pub struct ClientBuilder {
    api_key: String,
    base_url: url::Url,
    timeout: Option<std::time::Duration>,
    user_agent: Option<String>,
}

impl ClientBuilder {
    pub(crate) fn new(api_key: impl Into<String>, base_url: url::Url) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: http::normalize_base_url(base_url),
            timeout: None,
            user_agent: None,
        }
    }

    /// Sets a request timeout applied to all requests.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets a custom user agent.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Builds the client.
    pub fn build(self) -> Result<Client> {
        use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();

        // Authorization: Bearer <api_key>
        let auth_value = HeaderValue::from_str(&format!("Bearer {}", self.api_key))
            .map_err(|e| Error::InvalidHeaderValue(e.to_string()))?;
        headers.insert(AUTHORIZATION, auth_value);

        // Default Content-Type
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let mut builder = reqwest::Client::builder().default_headers(headers);

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(ua) = &self.user_agent {
            builder = builder.user_agent(ua.clone());
        }

        let http = builder.build()?;

        Ok(Client {
            base_url: self.base_url,
            http,
        })
    }
}
