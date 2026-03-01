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

    #[error("streaming error: {0}")]
    Streaming(#[from] StreamingError),
}

/// Errors specific to SSE streaming.
#[derive(Debug, thiserror::Error)]
pub enum StreamingError {
    #[error("unexpected content-type: expected text/event-stream, got {got:?}")]
    UnexpectedContentType { got: Option<String> },

    #[error("event too large: exceeded {limit_bytes} byte limit")]
    EventTooLarge { limit_bytes: usize },

    #[error("type mismatch: SSE event field {event:?} disagrees with JSON type field {ty:?}")]
    TypeMismatch { event: String, ty: String },

    #[error("json decode error in SSE payload: {source}")]
    Json {
        source: serde_json::Error,
        payload: String,
    },

    #[error("invalid UTF-8 in SSE stream: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}
