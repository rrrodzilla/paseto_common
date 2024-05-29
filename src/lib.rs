// mod error;
// mod keys;
mod purpose;
mod traits;
mod version;

pub mod prelude {

    // pub use crate::error::PasetoKeyError;
    // pub use crate::keys::Key;
    pub mod purposes{
        pub use crate::purpose::Local;
        pub use crate::purpose::Public;
    }
    pub mod traits {
        pub use crate::traits::Nist;
        pub use crate::traits::Sodium;
        pub use crate::traits::{ImplicitAssertionCapable, PurposeTrait, VersionTrait};
    }
    pub mod versions{
        pub use crate::version::V1;
        pub use crate::version::V2;
        pub use crate::version::V3;
        pub use crate::version::V4;
    }
}
