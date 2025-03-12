// Answer 0

#[test]
fn test_source_with_io_error() {
    use std::io::ErrorKind;

    let error_impl = ErrorImpl {
        code: ErrorCode::Io(std::io::Error::new(ErrorKind::Other, "test error message")),
        line: 1,
        column: 1,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    let _source = error.source();
}

#[test]
fn test_source_with_io_error_kind_not_found() {
    use std::io::ErrorKind;

    let error_impl = ErrorImpl {
        code: ErrorCode::Io(std::io::Error::new(ErrorKind::NotFound, "file not found")),
        line: 2,
        column: 2,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    let _source = error.source();
}

