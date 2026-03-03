//! HTTP client for LLM APIs.
//!
//! This module is behind the Cargo feature `client`.

pub mod endpoints;

mod builder;
mod error;
mod http;

pub use builder::ClientBuilder;
pub use endpoints::responses::ResponseEventStream;
pub use error::{Error, Result, StreamingError};

/// Marker type for the Responses API.
///
/// A [`Client<ResponsesApi>`] uses `Authorization: Bearer` authentication
/// and exposes the [`.responses()`](Client::responses) endpoint group.
#[derive(Clone, Copy, Debug)]
pub struct ResponsesApi;

/// Marker type for the Messages API.
///
/// A [`Client<MessagesApi>`] uses `x-api-key` authentication
/// and exposes the [`.messages()`](Client::messages) endpoint group.
#[derive(Clone, Copy, Debug)]
pub struct MessagesApi;

/// Reqwest-based API client, parameterized by API marker type.
///
/// Use [`ClientBuilder::responses`] or [`ClientBuilder::messages`] to
/// construct a client for the desired API surface. The type parameter
/// restricts which endpoint methods are available at compile time.
///
/// # Compile-time safety
///
/// A `Client<ResponsesApi>` cannot call `.messages()`:
///
/// ```compile_fail
/// fn test(client: schelm_http::client::Client<schelm_http::client::ResponsesApi>) {
///     client.messages(); // ERROR: no method `messages` on Client<ResponsesApi>
/// }
/// ```
///
/// A `Client<MessagesApi>` cannot call `.responses()`:
///
/// ```compile_fail
/// fn test(client: schelm_http::client::Client<schelm_http::client::MessagesApi>) {
///     client.responses(); // ERROR: no method `responses` on Client<MessagesApi>
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Client<A> {
    base_url: url::Url,
    http: reqwest::Client,
    _api: std::marker::PhantomData<A>,
}

impl Client<ResponsesApi> {
    /// Access the Responses endpoint group.
    pub fn responses(&self) -> endpoints::responses::Responses<'_> {
        endpoints::responses::Responses::new(self)
    }
}

impl Client<MessagesApi> {
    /// Access the Messages endpoint group.
    pub fn messages(&self) -> endpoints::messages::Messages<'_> {
        endpoints::messages::Messages::new(self)
    }
}

impl<A> Client<A> {
    pub(crate) fn http(&self) -> &reqwest::Client {
        &self.http
    }

    pub(crate) fn endpoint_url(&self, path: &str) -> Result<url::Url> {
        http::join(&self.base_url, path)
    }
}
