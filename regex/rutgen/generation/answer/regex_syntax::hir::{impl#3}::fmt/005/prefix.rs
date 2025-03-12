// Answer 0

#[test]
fn test_invalid_line_terminator() {
    use crate::ErrorKind;

    let error_kind = ErrorKind::InvalidLineTerminator;
    let mut formatter = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut formatter);
}

#[test]
fn test_invalid_line_terminator_display() {
    use crate::ErrorKind;

    let error_kind = ErrorKind::InvalidLineTerminator;
    let mut formatter = core::fmt::Formatter::new();
    let _ = core::fmt::Display::fmt(&error_kind, &mut formatter);
}

