// Answer 0

#[test]
fn test_character_invalid_escape_sequence() {
    let cursor = Cursor { rest: "'\\x" };
    let _result = character(cursor);
}

#[test]
fn test_character_invalid_escape_sequence_with_u() {
    let cursor = Cursor { rest: "'\\u" };
    let _result = character(cursor);
}

#[test]
fn test_character_invalid_escape_sequence_with_n() {
    let cursor = Cursor { rest: "'\\n" };
    let _result = character(cursor);
}

#[test]
fn test_character_invalid_escape_sequence_with_r() {
    let cursor = Cursor { rest: "'\\r" };
    let _result = character(cursor);
}

#[test]
fn test_character_invalid_escape_sequence_with_t() {
    let cursor = Cursor { rest: "'\\t" };
    let _result = character(cursor);
}

#[test]
fn test_character_invalid_escape_sequence_with_backslash() {
    let cursor = Cursor { rest: "'\\\\" };
    let _result = character(cursor);
}

#[test]
fn test_character_invalid_escape_sequence_with_zero() {
    let cursor = Cursor { rest: "'\\0" };
    let _result = character(cursor);
}

#[test]
fn test_character_invalid_escape_sequence_with_single_quote() {
    let cursor = Cursor { rest: "'\\'" };
    let _result = character(cursor);
}

#[test]
fn test_character_invalid_escape_sequence_with_double_quote() {
    let cursor = Cursor { rest: "'\\\"'" };
    let _result = character(cursor);
}

