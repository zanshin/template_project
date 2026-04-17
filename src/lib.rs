//! Core library for template_project.
//!
//! Re-exports error types and utility modules for use by the binary and tests.
pub mod error;
pub mod messages;

pub use error::{Error, Result};
