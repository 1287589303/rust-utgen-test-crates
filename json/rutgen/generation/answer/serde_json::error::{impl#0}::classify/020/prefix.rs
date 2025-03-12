// Answer 0

#[test]
fn test_classify_eof_while_parsing_value() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    enum ErrorCode {
        EofWhileParsingValue,
    }

    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingValue,
            line: 1,
            column: 1,
        }),
    };

    let category = error.classify();
}

#[test]
fn test_classify_eof_while_parsing_list() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    enum ErrorCode {
        EofWhileParsingList,
    }

    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingList,
            line: 1,
            column: 1,
        }),
    };

    let category = error.classify();
}

#[test]
fn test_classify_eof_while_parsing_string() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    enum ErrorCode {
        EofWhileParsingString,
    }

    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingString,
            line: 1,
            column: 1,
        }),
    };

    let category = error.classify();
}

#[test]
fn test_classify_eof_while_parsing_object() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    enum ErrorCode {
        EofWhileParsingObject,
    }

    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingObject,
            line: 1,
            column: 1,
        }),
    };

    let category = error.classify();
}

