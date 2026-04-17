//! Application entry point.
//!
//! Thin binary wrapper that wires together logging and error handling.
use log::{error, info, warn};
use template_project::{messages, Error, Result};

fn main() {
    messages::setup();
    info!("This is an info message");
    warn!("This is a warn message");
    error!("This is an error message");

    match get_an_error() {
        Ok(_) => info!("get_an_error had no error"),
        Err(_) => error!("get_an_error did have an error"),
    }
}

/// Always returns an [`Error::UsageInvalid`] to demonstrate error propagation.
///
/// # Errors
///
/// Returns [`Error::UsageInvalid`] unconditionally.
fn get_an_error() -> Result<()> {
    Err(Error::UsageInvalid)
}
