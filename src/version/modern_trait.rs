//! Defines the Modern trait for recent PASETO versions.

use crate::version::VersionTrait;

/// Marks PASETO versions supporting implicit assertions.
///
/// This trait applies to v3 and v4 PASETO tokens.
pub trait Modern: VersionTrait {}