//! Integration tests for template_project.
//!
//! Exercises the public API through the library crate boundary.
use template_project::Error;

#[test]
fn usage_invalid_display() {
    let e = Error::UsageInvalid;
    assert_eq!(
        e.to_string(),
        "invalid usage: unsupported argument or input combination"
    );
}

#[test]
fn io_error_display() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let e = Error::Io(io_err);
    assert!(e.to_string().starts_with("I/O error:"));
}

#[test]
fn io_error_converts_via_from() {
    let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
    let e: template_project::Error = io_err.into();
    assert!(matches!(e, template_project::Error::Io(_)));
}
