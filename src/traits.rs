//use base64::{decode_config, encode_config, DecodeError, URL_SAFE_NO_PAD};
//use ring::constant_time::verify_slices_are_equal as ConstantTimeEquals;
use std::fmt::Display;


//marker traits
/// Used by marker traits to determine at compile time which PASETO version the user is attempting to use
pub trait VersionTrait: Display + Default + AsRef<str> {}
/// Used by marker traits to determine at compile time which PASETO purpose the user is attempting to use
pub trait PurposeTrait: Display + Default + AsRef<str> {}
pub trait Nist: VersionTrait + private::Sealed {}
/// A marker trait used to determine if the PASETO token version is capable of using an implicit
/// assertion. Currently this applies only to V3/V4 PASETO tokens
pub trait ImplicitAssertionCapable: VersionTrait {}
pub trait Sodium: VersionTrait + private::Sealed {}
//pub(crate) trait Sealed {}
mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl super::Sodium for crate::version::V4 {}
    impl Sealed for crate::version::V4 {}
    impl super::Nist for crate::version::V3 {}
    impl Sealed for crate::version::V3 {}
    impl super::Sodium for crate::version::V2 {}
    impl Sealed for crate::version::V2 {}
    impl super::Nist for crate::version::V1 {}
    impl Sealed for crate::version::V1 {}
    //    impl<T: super::Nist> Sealed for T {}
}
