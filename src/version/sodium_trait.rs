//! Defines the Sodium trait for PASETO versions using libsodium.

use crate::version::VersionTrait;

/// Marks PASETO versions implemented with libsodium.
///
/// This trait extends `VersionTrait` and is sealed to prevent
/// external implementation.
pub trait Sodium: VersionTrait + crate::traits::private::Sealed {}