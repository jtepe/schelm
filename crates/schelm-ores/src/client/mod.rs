//! HTTP client for the OpenResponses API.
//!
//! This module is behind the Cargo feature `client`.

pub mod endpoints;

mod builder;
mod error;
mod http;
pub(crate) mod sse;

pub use builder::ClientBuilder;
pub use error::{Error, Result, StreamingError};
pub use sse::ResponseEventStream;

/// Reqwest-based API client.
#[derive(Clone, Debug)]
pub struct Client {
    base_url: url::Url,
    http: reqwest::Client,
}

impl Client {
    /// Creates a [`ClientBuilder`] with required fields.
    pub fn builder(api_key: impl Into<String>, base_url: url::Url) -> ClientBuilder {
        ClientBuilder::new(api_key, base_url)
    }

    /// Access the Responses endpoint group.
    pub fn responses(&self) -> endpoints::responses::Responses<'_> {
        endpoints::responses::Responses::new(self)
    }

    pub(crate) fn http(&self) -> &reqwest::Client {
        &self.http
    }

    pub(crate) fn endpoint_url(&self, path: &str) -> Result<url::Url> {
        http::join(&self.base_url, path)
    }
}
