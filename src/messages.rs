//! Logging initialization
//!
//! Call [`setup`] once at program startup to configure a [`simplelog`] combined
//! logger that writes to both the terminal and `app.log`.
use std::fs::File;

use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

/// Initialises the global logger.
///
/// Attaches two sinks:
/// - **Terminal** - colored output at [`LevelFilter::Info`] and above.
/// - **File** - plain-text output to `app.log` at the same level.
///
/// # Panics
///
/// Panics if `app.log` cannot be created or if the logger has already been
/// initialised.
pub fn setup() {
    let config = ConfigBuilder::new().set_time_format_rfc3339().build();
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, config, File::create("app.log").unwrap()),
    ])
    .unwrap();
}
