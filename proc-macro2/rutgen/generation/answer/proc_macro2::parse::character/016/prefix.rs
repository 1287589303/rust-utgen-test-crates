// Answer 0

#[test]
fn test_character_with_single_quote_and_escape_sequence() {
    let cursor = Cursor { rest: "'\\n'".to_string().as_str() };
    let _result = character(cursor);
}

#[test]
fn test_character_with_single_quote_and_unicode_escape() {
    let cursor = Cursor { rest: "'\\u1234'".to_string().as_str() };
    let _result = character(cursor);
}

#[test]
fn test_character_with_single_quote_and_hex_escape() {
    let cursor = Cursor { rest: "'\\xFF'".to_string().as_str() };
    let _result = character(cursor);
}

#[test]
fn test_character_with_single_quote_and_single_quote_escape() {
    let cursor = Cursor { rest: "'\\''".to_string().as_str() };
    let _result = character(cursor);
}

#[test]
fn test_character_with_single_quote_and_double_quote_escape() {
    let cursor = Cursor { rest: "'\\\"'".to_string().as_str() };
    let _result = character(cursor);
}

