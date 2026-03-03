//! Builder for [`Client`] with provider presets.

use std::marker::PhantomData;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

use crate::client::{Client, Error, MessagesApi, ResponsesApi, Result, http};

/// Builder for [`Client`].
///
/// Use [`responses`](Self::responses) or [`messages`](Self::messages) to
/// construct a builder for the desired API surface. The type parameter is
/// carried through to the resulting [`Client`], restricting which endpoint
/// methods are available at compile time.
#[derive(Debug)]
pub struct ClientBuilder<A> {
    api_key: String,
    base_url: url::Url,
    timeout: Option<std::time::Duration>,
    user_agent: Option<String>,
    _api: PhantomData<A>,
}

impl ClientBuilder<ResponsesApi> {
    /// Creates a builder pre-configured for the **Responses** API.
    ///
    /// The resulting client uses `Authorization: Bearer <api_key>` and
    /// only exposes the `.responses()` endpoint group.
    pub fn responses(api_key: impl Into<String>, base_url: url::Url) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: http::normalize_base_url(base_url),
            timeout: None,
            user_agent: None,
            _api: PhantomData,
        }
    }

    /// Builds the client with `Authorization: Bearer` authentication.
    pub fn build(self) -> Result<Client<ResponsesApi>> {
        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {}", self.api_key))
            .map_err(|e| Error::InvalidHeaderValue(e.to_string()))?;
        headers.insert(AUTHORIZATION, auth_value);
        self.build_with_headers(headers)
    }
}

impl ClientBuilder<MessagesApi> {
    /// Creates a builder pre-configured for the **Messages** API.
    ///
    /// The resulting client uses `x-api-key: <api_key>` and only exposes
    /// the `.messages()` endpoint group.
    pub fn messages(api_key: impl Into<String>, base_url: url::Url) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: http::normalize_base_url(base_url),
            timeout: None,
            user_agent: None,
            _api: PhantomData,
        }
    }

    /// Builds the client with `x-api-key` authentication.
    pub fn build(self) -> Result<Client<MessagesApi>> {
        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&self.api_key)
            .map_err(|e| Error::InvalidHeaderValue(e.to_string()))?;
        headers.insert("x-api-key", auth_value);
        self.build_with_headers(headers)
    }
}

impl<A> ClientBuilder<A> {
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

    /// Shared build logic: applies content-type, timeout, user-agent, and
    /// constructs the underlying `reqwest::Client`.
    fn build_with_headers(self, mut headers: HeaderMap) -> Result<Client<A>> {
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
            _api: PhantomData,
        })
    }
}
