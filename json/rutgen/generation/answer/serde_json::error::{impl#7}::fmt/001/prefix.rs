// Answer 0

#[test]
fn test_fmt_with_io_error() {
    let error_impl = ErrorImpl {
        code: ErrorCode::Io(io::Error::new(io::ErrorKind::Other, "test error")),
        line: 1,
        column: 1,
    };
    let error = Error { err: Box::new(error_impl) };
    let _ = format!("{}", error);
}

#[test]
fn test_fmt_with_eof_while_parsing_list() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingList,
        line: 999,
        column: 999,
    };
    let error = Error { err: Box::new(error_impl) };
    let _ = format!("{}", error);
}

#[test]
fn test_fmt_with_message_empty_string() {
    let error_impl = ErrorImpl {
        code: ErrorCode::Message(Box::from("")),
        line: 500,
        column: 100,
    };
    let error = Error { err: Box::new(error_impl) };
    let _ = format!("{}", error);
}

#[test]
fn test_fmt_with_message_long_string() {
    let long_message = "a".repeat(256); // long string
    let error_impl = ErrorImpl {
        code: ErrorCode::Message(Box::from(long_message)),
        line: 250,
        column: 250,
    };
    let error = Error { err: Box::new(error_impl) };
    let _ = format!("{}", error);
}

#[test]
fn test_fmt_with_eof_while_parsing_object() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingObject,
        line: 1000,
        column: 1000,
    };
    let error = Error { err: Box::new(error_impl) };
    let _ = format!("{}", error);
}

#[test]
fn test_fmt_with_expected_some_value() {
    let error_impl = ErrorImpl {
        code: ErrorCode::ExpectedSomeValue,
        line: 10,
        column: 20,
    };
    let error = Error { err: Box::new(error_impl) };
    let _ = format!("{}", error);
}

#[test]
fn test_fmt_with_trailing_comma() {
    let error_impl = ErrorImpl {
        code: ErrorCode::TrailingComma,
        line: 300,
        column: 30,
    };
    let error = Error { err: Box::new(error_impl) };
    let _ = format!("{}", error);
}

#[test]
fn test_fmt_with_number_out_of_range() {
    let error_impl = ErrorImpl {
        code: ErrorCode::NumberOutOfRange,
        line: 150,
        column: 75,
    };
    let error = Error { err: Box::new(error_impl) };
    let _ = format!("{}", error);
}

