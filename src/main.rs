//! Application entry point.
//!
//! Wires together logging ([`messages`]) and error handling ([`error`]), then
//! runs the program logic.
pub use self::error::{Error, Result};
use log::{error, info, warn};

mod error;
mod messages;

fn main() {
    messages::setup();
    info!("This is an info message");
    warn!("This is an warn message");
    error!("This is an error message");

    match get_an_error() {
        Ok(_) => info!("get_an_error had no error"),
        Err(_) => error!("get_an_error did have an error"),
    }
}

/// Always returns an [`Error::UsageInvalid`] to demonstrate error propagatio.
///
/// # Errors
///
/// Returns [`Error::UsageInvalid`] unconditionally.
fn get_an_error() -> Result<()> {
    Err(Error::UsageInvalid)
}
