// Answer 0

#[test]
fn test_classify_io_error() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    enum ErrorCode {
        Io(String),
    }

    let err_impl = ErrorImpl {
        code: ErrorCode::Io(String::from("IO error example")),
        line: 0,
        column: 0,
    };
    
    let error = Error {
        err: Box::new(err_impl),
    };

    let _category = error.classify();
}

#[test]
fn test_classify_io_error_with_other_data() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    enum ErrorCode {
        Io(String),
    }

    let err_impl = ErrorImpl {
        code: ErrorCode::Io(String::from("Another IO error")),
        line: 1,
        column: 2,
    };

    let error = Error {
        err: Box::new(err_impl),
    };

    let _category = error.classify();
}

