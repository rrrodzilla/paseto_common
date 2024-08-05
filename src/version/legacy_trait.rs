//! Defines the Modern trait for legacy PASETO versions.

use crate::version::VersionTrait;

/// Marks PASETO versions as NOT supporting implicit assertions.
///
/// This trait applies to v1 and v2 PASETO tokens.
pub trait Legacy: VersionTrait {}