//! Defines private traits and implementations for PASETO versions.

pub(crate) mod private {
    use crate::version::{Nist, Sodium};

    /// Sealed trait to prevent external implementation of certain traits.
    pub trait Sealed {}

    /// Implements Sodium and Sealed traits for PASETO v4.
    impl Sodium for crate::version::V4 {}
    impl Sealed for crate::version::V4 {}

    /// Implements Nist and Sealed traits for PASETO v3.
    impl Nist for crate::version::V3 {}
    impl Sealed for crate::version::V3 {}

    /// Implements Sodium and Sealed traits for PASETO v2.
    impl Sodium for crate::version::V2 {}
    impl Sealed for crate::version::V2 {}

    /// Implements Nist and Sealed traits for PASETO v1.
    impl Nist for crate::version::V1 {}
    impl Sealed for crate::version::V1 {}
}