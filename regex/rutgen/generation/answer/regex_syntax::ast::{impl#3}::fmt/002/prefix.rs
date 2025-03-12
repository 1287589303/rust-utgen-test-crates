// Answer 0

#[test]
fn test_unsupported_backreference_display() {
    let error_kind = ErrorKind::UnsupportedBackreference;
    let mut buffer = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut buffer);
}

#[test]
fn test_unsupported_backreference_formatting() {
    let error_kind = ErrorKind::UnsupportedBackreference;
    let mut buffer = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut buffer);
}

#[test]
fn test_unsupported_backreference_string_output() {
    let error_kind = ErrorKind::UnsupportedBackreference;
    let mut buffer = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut buffer);
}

