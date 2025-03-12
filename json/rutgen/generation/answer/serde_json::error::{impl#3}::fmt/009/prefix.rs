// Answer 0

#[test]
fn test_error_code_control_character_while_parsing_string_display() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error_code);
}

#[test]
fn test_error_code_control_character_while_parsing_string_format() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let _ = error_code.fmt(&mut fmt::Formatter::new());
}

