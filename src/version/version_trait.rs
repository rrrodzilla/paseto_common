use std::fmt::Display;

/// Used by marker traits to determine at compile time which PASETO version the user is attempting to use
pub trait VersionTrait: Display + Default + AsRef<str> {}

