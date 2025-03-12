// Answer 0

#[test]
fn test_classify_eof_while_parsing_list() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestError {
        err: Box<TestErrorImpl>,
    }

    let test_error = TestError {
        err: Box::new(TestErrorImpl {
            code: ErrorCode::EofWhileParsingList,
            line: 1,
            column: 5,
        }),
    };

    let category = test_error.classify();
}

#[test]
fn test_classify_eof_while_parsing_object() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestError {
        err: Box<TestErrorImpl>,
    }

    let test_error = TestError {
        err: Box::new(TestErrorImpl {
            code: ErrorCode::EofWhileParsingObject,
            line: 2,
            column: 10,
        }),
    };

    let category = test_error.classify();
}

#[test]
fn test_classify_eof_while_parsing_string() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestError {
        err: Box<TestErrorImpl>,
    }

    let test_error = TestError {
        err: Box::new(TestErrorImpl {
            code: ErrorCode::EofWhileParsingString,
            line: 3,
            column: 15,
        }),
    };

    let category = test_error.classify();
}

#[test]
fn test_classify_eof_while_parsing_value() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestError {
        err: Box<TestErrorImpl>,
    }

    let test_error = TestError {
        err: Box::new(TestErrorImpl {
            code: ErrorCode::EofWhileParsingValue,
            line: 4,
            column: 20,
        }),
    };

    let category = test_error.classify();
}

