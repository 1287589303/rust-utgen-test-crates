// Answer 0

#[test]
fn test_fmt_key_must_be_a_string() {
    struct TestError {}

    impl Debug for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Test error")
        }
    }

    let error_code = ErrorCode::KeyMustBeAString;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_message() {
    let message = Box::<str>::from("Test message");
    let error_code = ErrorCode::Message(message);
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_numeric_key() {
    let error_code = ErrorCode::ExpectedNumericKey;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

