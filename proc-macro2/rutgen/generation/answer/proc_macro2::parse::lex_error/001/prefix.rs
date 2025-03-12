// Answer 0

#[test]
fn test_lex_error_empty_cursor() {
    let cursor = Cursor { rest: "", off: 0 };
    let _ = lex_error(cursor);
}

#[test]
fn test_lex_error_single_character_cursor() {
    let cursor = Cursor { rest: "a", off: 1 };
    let _ = lex_error(cursor);
}

#[test]
fn test_lex_error_multiple_characters_cursor() {
    let cursor = Cursor { rest: "hello", off: 10 };
    let _ = lex_error(cursor);
}

#[test]
fn test_lex_error_max_offset() {
    let cursor = Cursor { rest: "text", off: u32::MAX };
    let _ = lex_error(cursor);
}

#[cfg(not(span_locations))]
#[test]
fn test_lex_error_disabled_span_locations() {
    let cursor = Cursor { rest: "some text", off: 5 };
    let _ = lex_error(cursor);
}

