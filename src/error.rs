//! Error types for this application.
//!
//! Defines the top-level [`Error`] enum and a [`Result`] type alias so that
//! call sites can write `Result<T>` instead of `Result<T, Error>`.

/// Convenient alias for `Result<T, Error>` scoped to this crate's error type.
pub type Result<T> = core::result::Result<T, Error>;

/// Top-level error type for this application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The caller supplied arguments or input in an unsupported combination.
    #[error("invalid usage: unsupported argument or input combination")]
    UsageInvalid,
    /// An I/O operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
