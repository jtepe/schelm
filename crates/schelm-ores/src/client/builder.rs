//! Builder for [`Client`] with provider presets and custom configuration.

use crate::client::{Client, Error, Result, http};

/// Builder for [`Client`].
///
/// Use one of the provider presets ([`responses`](Self::responses),
/// [`messages`](Self::messages)) or the generic [`custom`](Self::custom)
/// constructor to get started. Each preset applies sensible defaults for
/// authentication headers, content-type, etc.
#[derive(Debug)]
pub struct ClientBuilder {
    api_key: String,
    base_url: url::Url,
    timeout: Option<std::time::Duration>,
    user_agent: Option<String>,
    auth_scheme: AuthScheme,
}

/// Controls how the API key is sent in HTTP requests.
#[derive(Debug, Clone)]
enum AuthScheme {
    /// `Authorization: Bearer <key>`
    Bearer,
    /// `x-api-key: <key>`
    XApiKey,
}

impl ClientBuilder {
    /// Creates a builder pre-configured for the **Responses** API.
    ///
    /// Defaults:
    /// - `Authorization: Bearer <api_key>`
    /// - `Content-Type: application/json`
    pub fn responses(api_key: impl Into<String>, base_url: url::Url) -> Self {
        Self::new(api_key, base_url, AuthScheme::Bearer)
    }

    /// Creates a builder pre-configured for the **Messages** API.
    ///
    /// Defaults:
    /// - `x-api-key: <api_key>`
    /// - `Content-Type: application/json`
    pub fn messages(api_key: impl Into<String>, base_url: url::Url) -> Self {
        Self::new(api_key, base_url, AuthScheme::XApiKey)
    }

    /// Creates a builder with explicit control over the auth scheme.
    ///
    /// This generic path allows future providers to be added without another
    /// builder redesign.
    pub fn custom(api_key: impl Into<String>, base_url: url::Url) -> Self {
        Self::new(api_key, base_url, AuthScheme::Bearer)
    }

    fn new(api_key: impl Into<String>, base_url: url::Url, auth_scheme: AuthScheme) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: http::normalize_base_url(base_url),
            timeout: None,
            user_agent: None,
            auth_scheme,
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

    /// Overrides the default auth scheme to use `Authorization: Bearer`.
    pub fn auth_bearer(mut self) -> Self {
        self.auth_scheme = AuthScheme::Bearer;
        self
    }

    /// Overrides the default auth scheme to use `x-api-key` header.
    pub fn auth_x_api_key(mut self) -> Self {
        self.auth_scheme = AuthScheme::XApiKey;
        self
    }

    /// Builds the client.
    pub fn build(self) -> Result<Client> {
        use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();

        // Apply auth based on the chosen scheme
        match self.auth_scheme {
            AuthScheme::Bearer => {
                let auth_value = HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                    .map_err(|e| Error::InvalidHeaderValue(e.to_string()))?;
                headers.insert(AUTHORIZATION, auth_value);
            }
            AuthScheme::XApiKey => {
                let auth_value = HeaderValue::from_str(&self.api_key)
                    .map_err(|e| Error::InvalidHeaderValue(e.to_string()))?;
                headers.insert("x-api-key", auth_value);
            }
        }

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
