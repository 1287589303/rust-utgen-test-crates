// Answer 0

#[test]
fn test_error_code_message() {
    let msg = Box::<str>::from("An error occurred");
    let error_code = ErrorCode::Message(msg);
    let mut f = fmt::Formatter::default();
    error_code.fmt(&mut f);
}

#[test]
fn test_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut f = fmt::Formatter::default();
    error_code.fmt(&mut f);
}

#[test]
fn test_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut f = fmt::Formatter::default();
    error_code.fmt(&mut f);
}

#[test]
fn test_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut f = fmt::Formatter::default();
    error_code.fmt(&mut f);
}

#[test]
fn test_trailing_characters() {
    let error_code = ErrorCode::TrailingCharacters;
    let mut f = fmt::Formatter::default();
    error_code.fmt(&mut f);
}

#[test]
fn test_recursion_limit_exceeded() {
    let error_code = ErrorCode::RecursionLimitExceeded;
    let mut f = fmt::Formatter::default();
    error_code.fmt(&mut f);
}

