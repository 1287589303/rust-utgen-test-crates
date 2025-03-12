// Answer 0

#[test]
fn test_error_kind_decimal_empty() {
    let error = ErrorKind::DecimalEmpty;
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_error_kind_decimal_empty_multiple() {
    let error = ErrorKind::DecimalEmpty;
    let mut buffer1 = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer1);
    
    let mut buffer2 = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer2);
}

