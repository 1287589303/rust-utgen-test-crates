// Answer 0

#[test]
fn test_error_kind_decimal_invalid() {
    let error = ErrorKind::DecimalInvalid;
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_error_kind_decimal_empty() {
    let error = ErrorKind::DecimalEmpty;
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

