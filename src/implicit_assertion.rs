use std::fmt;
use std::ops::Deref;

/// Unencrypted but authenticated data (like the optional footer), but is NOT stored in the PASETO token (thus, implicit) and MUST be asserted when verifying a token.
/// The main purpose for Implicit Assertions is to bind the token to some value that, due to business reasons, shouldn't ever be revealed publicly (i.e., a primary key or foreign key from a relational database table).
/// Implicit Assertions allow you to build systems that are impervious to Confused Deputy attacks without ever having to disclose these internal values.
///
#[derive(Default, Debug, Copy, Clone)]
pub struct ImplicitAssertion<'a>(&'a str);

impl<'a> Deref for ImplicitAssertion<'a> {
    type Target = [u8];

    fn deref(&self) -> &'a Self::Target {
        self.0.as_bytes()
    }
}

impl<'a> AsRef<str> for ImplicitAssertion<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
impl<'a> From<&'a str> for ImplicitAssertion<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}
impl<'a> fmt::Display for ImplicitAssertion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<'a> PartialEq for ImplicitAssertion<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<'a> Eq for ImplicitAssertion<'a> {}
