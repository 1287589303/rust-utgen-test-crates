// Answer 0

#[test]
fn test_error_code_fmt_expected_some_ident() {
    let error_code = ErrorCode::ExpectedSomeIdent;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_fmt_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_fmt_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_fmt_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_fmt_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

