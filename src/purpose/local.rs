//! Defines the Local purpose for PASETO tokens.

use std::fmt;
use std::fmt::Display;

use crate::purpose::PurposeTrait;

/// Represents the Local purpose in PASETO.
///
/// Local tokens are encrypted and authenticated using symmetric keys.
#[derive(Debug, Clone, Copy)]
pub struct Local(&'static str);

impl PurposeTrait for Local {}

impl Default for Local {
    /// Creates a new Local purpose with the default value "local".
    fn default() -> Self {
        Self("local")
    }
}

impl AsRef<str> for Local {
    /// Returns the string representation of the Local purpose.
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Display for Local {
    /// Formats the Local purpose as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}