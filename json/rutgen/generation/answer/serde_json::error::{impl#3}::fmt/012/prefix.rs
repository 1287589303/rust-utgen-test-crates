// Answer 0

#[test]
fn test_error_code_display_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_display_invalid_escape() {
    let error_code = ErrorCode::InvalidEscape;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_display_message() {
    let error_code = ErrorCode::Message(Box::from("Custom error message"));
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_display_number_out_of_range() {
    let error_code = ErrorCode::NumberOutOfRange;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_display_control_character() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

