// Answer 0

#[test]
fn test_capture_limit_exceeded() {
    use crate::ErrorKind;

    let error = ErrorKind::CaptureLimitExceeded;
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_class_escape_invalid() {
    use crate::ErrorKind;

    let error = ErrorKind::ClassEscapeInvalid;
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_class_range_invalid() {
    use crate::ErrorKind;

    let error = ErrorKind::ClassRangeInvalid;
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_class_range_literal() {
    use crate::ErrorKind;

    let error = ErrorKind::ClassRangeLiteral;
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_class_unclosed() {
    use crate::ErrorKind;

    let error = ErrorKind::ClassUnclosed;
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

