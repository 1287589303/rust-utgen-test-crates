// Answer 0

#[test]
fn test_source_with_message_error_code() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message("A syntax error occurred".into()),
            line: 1,
            column: 5,
        }),
    };
    let _ = error.source();
}

#[test]
fn test_source_with_eof_while_parsing_list() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingList,
            line: 2,
            column: 10,
        }),
    };
    let _ = error.source();
}

#[test]
fn test_source_with_eof_while_parsing_object() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingObject,
            line: 3,
            column: 15,
        }),
    };
    let _ = error.source();
}

#[test]
fn test_source_with_eof_while_parsing_string() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingString,
            line: 4,
            column: 20,
        }),
    };
    let _ = error.source();
}

#[test]
fn test_source_with_expected_colon_error_code() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::ExpectedColon,
            line: 5,
            column: 25,
        }),
    };
    let _ = error.source();
}

#[test]
fn test_source_with_trailing_characters_error_code() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::TrailingCharacters,
            line: 6,
            column: 30,
        }),
    };
    let _ = error.source();
}

