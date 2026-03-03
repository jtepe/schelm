//! Messages API endpoint.
//!
//! This module is a placeholder for the future Messages API endpoint
//! implementation. The [`Messages`] struct is wired up to
//! [`Client<MessagesApi>`](crate::client::Client) so that the type system
//! is ready for future endpoint methods.

use crate::client::{Client, MessagesApi};

/// Messages endpoint group (placeholder).
#[derive(Clone, Copy, Debug)]
pub struct Messages<'a> {
    #[allow(dead_code)]
    client: &'a Client<MessagesApi>,
}

impl<'a> Messages<'a> {
    pub(crate) fn new(client: &'a Client<MessagesApi>) -> Self {
        Self { client }
    }
}
