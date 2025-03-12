// Answer 0

#[test]
fn test_unsupported_look_around() {
    let error_kind = ErrorKind::UnsupportedLookAround;
    let mut formatter = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut formatter);
}

