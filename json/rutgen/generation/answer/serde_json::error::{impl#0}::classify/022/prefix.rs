// Answer 0

#[test]
fn test_classify_eof_while_parsing_object() {
    struct ErrorCodeWrapper {
        code: ErrorCode,
    }

    struct ErrorImplWrapper {
        code: ErrorCodeWrapper,
        line: usize,
        column: usize,
    }

    struct ErrorWrapper {
        err: Box<ErrorImplWrapper>,
    }

    let error = ErrorWrapper {
        err: Box::new(ErrorImplWrapper {
            code: ErrorCodeWrapper { code: ErrorCode::EofWhileParsingObject },
            line: 1,
            column: 1,
        }),
    };

    let _category = error.classify();
}

#[test]
fn test_classify_eof_while_parsing_value() {
    struct ErrorCodeWrapper {
        code: ErrorCode,
    }

    struct ErrorImplWrapper {
        code: ErrorCodeWrapper,
        line: usize,
        column: usize,
    }

    struct ErrorWrapper {
        err: Box<ErrorImplWrapper>,
    }

    let error = ErrorWrapper {
        err: Box::new(ErrorImplWrapper {
            code: ErrorCodeWrapper { code: ErrorCode::EofWhileParsingValue },
            line: 1,
            column: 1,
        }),
    };

    let _category = error.classify();
}

#[test]
fn test_classify_eof_while_parsing_list() {
    struct ErrorCodeWrapper {
        code: ErrorCode,
    }

    struct ErrorImplWrapper {
        code: ErrorCodeWrapper,
        line: usize,
        column: usize,
    }

    struct ErrorWrapper {
        err: Box<ErrorImplWrapper>,
    }

    let error = ErrorWrapper {
        err: Box::new(ErrorImplWrapper {
            code: ErrorCodeWrapper { code: ErrorCode::EofWhileParsingList },
            line: 1,
            column: 1,
        }),
    };

    let _category = error.classify();
}

#[test]
fn test_classify_eof_while_parsing_string() {
    struct ErrorCodeWrapper {
        code: ErrorCode,
    }

    struct ErrorImplWrapper {
        code: ErrorCodeWrapper,
        line: usize,
        column: usize,
    }

    struct ErrorWrapper {
        err: Box<ErrorImplWrapper>,
    }

    let error = ErrorWrapper {
        err: Box::new(ErrorImplWrapper {
            code: ErrorCodeWrapper { code: ErrorCode::EofWhileParsingString },
            line: 1,
            column: 1,
        }),
    };

    let _category = error.classify();
}

