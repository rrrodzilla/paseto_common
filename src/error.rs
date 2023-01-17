use thiserror::Error;

/// Potential errors from attempting to use a Key structure
#[derive(Debug, Error)]
pub enum PasetoKeyError {
    ///Unspecified error when attepting to use the system's RNG
    #[error("An unspecified RNG error occurred")]
    UnspecifiedError {
        ///Surfaces unspecified errors from ring
        #[from]
        source: ring::error::Unspecified,
    },
}
