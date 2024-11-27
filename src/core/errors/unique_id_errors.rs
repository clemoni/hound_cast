
use thiserror::Error;


/// Errors that can occur when working with `UniqueId`.
#[derive(Debug, Clone, Error)]
pub enum UniqueIdError {
    /// Error indicating that the prefix is missing.
    #[error("UniqueId: Prefix cannot be empty")]
    MissingPrefix,
    
    /// Error indicating an invalid UniqueId format.
    #[error("UniqueId: Invalid UniqueId format. Expected 3 parts but got {0}")]
    WrongFormat(usize),
}
