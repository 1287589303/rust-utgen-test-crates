// Answer 0

#[test]
fn test_io_error_kind_timed_out() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::Io(io::Error::new(ErrorKind::TimedOut, "timed out")),
        line: 1,
        column: 1,
    };

    let test_error = TestError {
        err: Box::new(error_impl),
    };

    let _ = test_error.err.code;
}

#[test]
fn test_io_error_kind_not_found() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::Io(io::Error::new(ErrorKind::NotFound, "not found")),
        line: 1,
        column: 1,
    };

    let test_error = TestError {
        err: Box::new(error_impl),
    };

    let _ = test_error.err.code;
}

#[test]
fn test_io_error_kind_permission_denied() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::Io(io::Error::new(ErrorKind::PermissionDenied, "permission denied")),
        line: 1,
        column: 1,
    };

    let test_error = TestError {
        err: Box::new(error_impl),
    };

    let _ = test_error.err.code;
}

#[test]
fn test_io_error_kind_broken_pipe() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::Io(io::Error::new(ErrorKind::BrokenPipe, "broken pipe")),
        line: 1,
        column: 1,
    };

    let test_error = TestError {
        err: Box::new(error_impl),
    };

    let _ = test_error.err.code;
}

