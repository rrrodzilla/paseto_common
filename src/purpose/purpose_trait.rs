use std::fmt::Display;

/// Used by marker traits to determine at compile time which PASETO purpose the user is attempting to use
pub trait PurposeTrait: Display + Default + AsRef<str> {}
