/// Result type used by the client.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("invalid header value: {0}")]
    InvalidHeaderValue(String),

    #[error("http error status {status}: {body}")]
    HttpStatus {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("streaming responses are not supported; do not set `stream=true`")]
    StreamingNotSupported,
}
