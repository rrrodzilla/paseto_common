mod error;
mod keys;
mod purpose;
mod traits;
mod version;

pub mod prelude {

    pub use crate::error::PasetoKeyError;
    pub use crate::keys::Key;
    #[cfg(feature = "local")]
    pub use crate::purpose::Local;
    #[cfg(feature = "public")]
    pub use crate::purpose::Public;
    #[cfg(any(feature = "v1", feature = "v3"))]
    pub use crate::traits::Nist;
    #[cfg(any(feature = "v2", feature = "v4"))]
    pub use crate::traits::Sodium;
    pub use crate::traits::{ImplicitAssertionCapable, PurposeTrait, VersionTrait};
    #[cfg(feature = "v1")]
    pub use crate::version::V1;
    #[cfg(feature = "v2")]
    pub use crate::version::V2;
    #[cfg(feature = "v3")]
    pub use crate::version::V3;
    #[cfg(feature = "v4")]
    pub use crate::version::V4;
}
