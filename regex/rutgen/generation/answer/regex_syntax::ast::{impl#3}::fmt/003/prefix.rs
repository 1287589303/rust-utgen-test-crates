// Answer 0

#[test]
fn test_unicode_class_invalid_display() {
    let error = ErrorKind::UnicodeClassInvalid;
    let mut output = core::fmt::Formatter::default();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_unicode_class_invalid() {
    let error = ErrorKind::UnicodeClassInvalid;
    let mut output = core::fmt::Formatter::default();
    let _ = error.fmt(&mut output);
}

