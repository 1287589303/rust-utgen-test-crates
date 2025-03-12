// Answer 0

#[test]
fn test_cooked_string_with_just_returning_err() {
    let input_str = "\r\nabc"; // Input starts with '\r' followed by '\n' and then more characters
    let cursor = Cursor { rest: input_str };
    let result = cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_multiple_newlines() {
    let input_str = "\r\n\r\n"; // Input contains multiple lines that match the preconditions
    let cursor = Cursor { rest: input_str };
    let result = cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_trailing_whitespace() {
    let input_str = "\r\n     "; // Input starts with '\r' followed by '\n' and ends with whitespace
    let cursor = Cursor { rest: input_str };
    let result = cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_extra_character_after_newline() {
    let input_str = "\r\nx"; // Input starts with '\r' followed by '\n', then a valid character 'x'  
    let cursor = Cursor { rest: input_str };
    let result = cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_only_return_char_and_newline() {
    let input_str = "\r"; // Input only has a return character expecting valid input (following lines must produce an error)
    let cursor = Cursor { rest: input_str };
    let result = cooked_string(cursor);
}

