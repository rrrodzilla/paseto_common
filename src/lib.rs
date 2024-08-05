//! Common types for PASETO (Platform-Agnostic Security Tokens) implementation.
//!
//! This crate provides modules for handling PASETO purposes and versions.
//! It implements types defined in the PASETO specification:
//! https://github.com/paseto-standard/paseto-spec
//!
//! # Modules
//!
//! - `purpose`: Defines PASETO purposes (Local, Public).
//! - `traits`: Contains trait definitions for PASETO operations.
//! - `version`: Specifies PASETO versions (v1, v2, v3, v4).
//!
//! # Prelude
//!
//! The `prelude` module re-exports common PASETO types:
//!
//! - `purposes`: PASETO purpose types and traits.
//! - `versions`: PASETO version types and traits.
//!
//! # Usage
//!
//! Import the prelude to access PASETO types:
//!
//! ```
//! use paseto_common::prelude::*;
//! ```

#![deny(missing_docs)]

mod purpose;

mod traits;

mod version;
mod header;
mod footer;
mod implicit_assertion;

/// Prelude module for convenient importing of common PASETO types.
pub mod purposes {
    pub use crate::purpose::{Local, Public, PurposeTrait};
}

/// Prelude module for convenient importing of common PASETO types.
pub mod versions {
    pub use crate::version::{Modern, Legacy, Nist, Sodium, V1, V2, V3, V4, VersionTrait};
}

/// Prelude module for convenient importing of common PASETO types.
pub mod prelude {
    /// Re-exports PASETO purpose types and traits.
    pub use super::header::*;
    pub use super::footer::*;
    pub use super::implicit_assertion::*;
    pub use super::purpose::*;
    /// Re-exports PASETO version types and traits.
    pub use super::version::*;
}