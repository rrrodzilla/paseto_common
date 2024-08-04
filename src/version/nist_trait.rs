//! Defines the Nist trait for PASETO versions using NIST standards.

use crate::version::version_trait::VersionTrait;

/// Marks PASETO versions implemented with NIST cryptographic standards.
///
/// This trait extends `VersionTrait` and is sealed to prevent
/// external implementation.
pub trait Nist: VersionTrait + crate::traits::private::Sealed {}