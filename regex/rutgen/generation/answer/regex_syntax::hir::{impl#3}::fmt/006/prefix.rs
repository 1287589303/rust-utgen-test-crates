// Answer 0

#[test]
fn test_error_kind_fmt_invalid_utf8() {
    let error_kind = crate::ErrorKind::InvalidUtf8;
    let mut buffer = core::fmt::Formatter::default();
    let _ = error_kind.fmt(&mut buffer);
}

#[test]
fn test_error_kind_display_invalid_utf8() {
    let error_kind = crate::ErrorKind::InvalidUtf8;
    let _ = format!("{}", error_kind);
}

