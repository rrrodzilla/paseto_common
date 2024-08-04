//! Defines the Public purpose for PASETO tokens.

use std::fmt;
use std::fmt::Display;

use crate::purpose::PurposeTrait;

/// Represents the Public purpose in PASETO.
///
/// Public tokens are signed and can be verified with a public key.
#[derive(Debug, Clone, Copy)]
pub struct Public(&'static str);

impl PurposeTrait for Public {}

impl AsRef<str> for Public {
    /// Returns the string representation of the Public purpose.
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Default for Public {
    /// Creates a new Public purpose with the default value "public".
    fn default() -> Self {
        Self("public")
    }
}

impl Display for Public {
    /// Formats the Public purpose as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}