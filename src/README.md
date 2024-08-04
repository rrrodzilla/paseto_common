# paseto_common

Common types for PASETO (Platform-Agnostic Security Tokens) implementation.

## Overview

`paseto_common` provides core types and traits for the PASETO protocol versions and purposes. It serves as a foundation
for the rusty_paseto family of crates.

## Features

- Defines PASETO versions (v1, v2, v3, v4)
- Implements PASETO purposes (Local, Public)
- Provides traits for version-specific operations (Nist, Sodium, Modern)
- Offers a prelude for easy importing of common types

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
paseto_common = "1.0.0-alpha.1"
```

## About the Author

This crate is maintained by Roland Rodriguez, a technologist with 30 years of industry experience. Roland is a former
SOA and cloud architect, and former Principal Technical Product Manager at AWS for the Rust Programming Language. He is
currently the owner and operator of Govcraft, building and consulting on modern cloud solutions and AI solutions.

For more information or to discuss consulting and support opportunities, visit https://www.govcraft.ai