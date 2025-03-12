// Answer 0

#[test]
fn test_error_code_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut f = fmt::Formatter::new();
    let _ = error_code.fmt(&mut f);
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut f = fmt::Formatter::new();
    let _ = error_code.fmt(&mut f);
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut f = fmt::Formatter::new();
    let _ = error_code.fmt(&mut f);
}

#[test]
fn test_error_code_expected_some_value() {
    let error_code = ErrorCode::ExpectedSomeValue;
    let mut f = fmt::Formatter::new();
    let _ = error_code.fmt(&mut f);
}

#[test]
fn test_error_code_recursion_limit_exceeded() {
    let error_code = ErrorCode::RecursionLimitExceeded;
    let mut f = fmt::Formatter::new();
    let _ = error_code.fmt(&mut f);
}

