// Answer 0

#[test]
fn test_is_syntax_expected_colon() {
    struct ErrorWrapper {
        err: Box<ErrorImpl>,
    }
    
    let error = ErrorWrapper {
        err: Box::new(ErrorImpl {
            code: ErrorCode::ExpectedColon,
            line: 1,
            column: 5,
        }),
    };
    
    assert!(error.is_syntax());
}

#[test]
fn test_is_syntax_invalid_number() {
    struct ErrorWrapper {
        err: Box<ErrorImpl>,
    }
    
    let error = ErrorWrapper {
        err: Box::new(ErrorImpl {
            code: ErrorCode::InvalidNumber,
            line: 2,
            column: 10,
        }),
    };
    
    assert!(error.is_syntax());
}

#[test]
fn test_is_syntax_other_syntax_errors() {
    struct ErrorWrapper {
        err: Box<ErrorImpl>,
    }
    
    let error_expected_list_comma = ErrorWrapper {
        err: Box::new(ErrorImpl {
            code: ErrorCode::ExpectedListCommaOrEnd,
            line: 3,
            column: 15,
        }),
    };
    assert!(error_expected_list_comma.is_syntax());
    
    let error_invalid_escape = ErrorWrapper {
        err: Box::new(ErrorImpl {
            code: ErrorCode::InvalidEscape,
            line: 4,
            column: 20,
        }),
    };
    assert!(error_invalid_escape.is_syntax());
}

#[test]
fn test_is_syntax_non_syntax_errors() {
    struct ErrorWrapper {
        err: Box<ErrorImpl>,
    }
    
    let error_io = ErrorWrapper {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(ErrorKind::Other),
            line: 5,
            column: 25,
        }),
    };
    assert!(!error_io.is_syntax());

    let error_eof = ErrorWrapper {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingValue,
            line: 6,
            column: 30,
        }),
    };
    assert!(!error_eof.is_syntax());

    let error_data = ErrorWrapper {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message("Sample error".to_string()),
            line: 7,
            column: 35,
        }),
    };
    assert!(!error_data.is_syntax());
}

