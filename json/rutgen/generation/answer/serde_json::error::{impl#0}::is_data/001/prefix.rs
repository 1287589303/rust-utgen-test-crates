// Answer 0

#[test]
fn test_is_data_with_message_error() {
    struct TestError;
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message("Test error".to_string()),
            line: 0,
            column: 0,
        }),
    };
    error.is_data();
}

#[test]
fn test_is_data_with_io_error() {
    struct TestError;
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(io::Error::new(io::ErrorKind::Other, "IO error")),
            line: 1,
            column: 1,
        }),
    };
    error.is_data();
}

#[test]
fn test_is_data_with_syntax_error() {
    struct TestError;
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::InvalidNumber,
            line: 2,
            column: 2,
        }),
    };
    error.is_data();
}

#[test]
fn test_is_data_with_data_error() {
    struct TestError;
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message("Semantic error".to_string()),
            line: 3,
            column: 3,
        }),
    };
    error.is_data();
}

#[test]
fn test_is_data_with_eof_error() {
    struct TestError;
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingValue,
            line: 4,
            column: 4,
        }),
    };
    error.is_data();
}

