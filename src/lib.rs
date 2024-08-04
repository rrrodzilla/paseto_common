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
//! use paseto_common::prelude::{purposes, versions};
//! ```

#![deny(missing_docs)]

mod purpose;

mod traits;

mod version;

/// Prelude module for convenient importing of common PASETO types.
pub mod prelude {
    /// Re-exports PASETO purpose types and traits.
    pub mod purposes {
        pub use crate::purpose::{Local, Public, PurposeTrait};
    }

    /// Re-exports PASETO version types and traits.
    pub mod versions {
        pub use crate::version::{Modern, Nist, Sodium, V1, V2, V3, V4, VersionTrait};
    }
}