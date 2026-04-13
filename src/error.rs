//! Error types for this application
//!
//! Defines the top-level ['Error'] enum and a ['Result'] type alias so that
//! call sites can write `Result<T>` instead of `Result<T, Error>`.

/// Convenient alias for `Result<T, Error>` scoped to this crate's error type.
pub type Result<T> = core::result::Result<T, Error>;

/// Top-level error type for this application.
#[derive(Debug)]
pub enum Error {
    /// The called supplied arguments or input in an upsupported combination.
    UsageInvalid,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
