//! API endpoint modules.
//!
//! Each sub-module owns the endpoint logic and streaming types for a single
//! API surface. Shared SSE transport infrastructure lives in [`sse_core`].

pub mod messages;
pub mod responses;
pub(crate) mod sse_core;
