// Answer 0

#[test]
fn test_fmt_expected_some_value() {
    let error_code = ErrorCode::ExpectedSomeValue;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_some_ident() {
    let error_code = ErrorCode::ExpectedSomeIdent;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

