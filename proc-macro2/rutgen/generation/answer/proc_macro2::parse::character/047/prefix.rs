// Answer 0

#[test]
fn test_character_valid_escape_sequence_backslash() {
    let cursor = Cursor { rest: "'\\n' some text" };
    let result = character(cursor);
}

#[test]
fn test_character_valid_escape_sequence_hex() {
    let cursor = Cursor { rest: "'\\x41' some text" }; // 'A'
    let result = character(cursor);
}

#[test]
fn test_character_valid_escape_sequence_unicode() {
    let cursor = Cursor { rest: "'\\u{1F600}' some text" }; // 😀
    let result = character(cursor);
}

#[test]
fn test_character_valid_escape_sequence_single_quote() {
    let cursor = Cursor { rest: "'\\'' some text" };
    let result = character(cursor);
}

#[test]
fn test_character_valid_escape_sequence_double_quote() {
    let cursor = Cursor { rest: "'\\\"' some text" };
    let result = character(cursor);
}

#[test]
fn test_character_character_after_escape_sequence() {
    let cursor = Cursor { rest: "'\\t' some text" };
    let result = character(cursor);
}

#[test]
fn test_character_rejection_on_final_advance() {
    let cursor = Cursor { rest: "'\\n" }; // Missing closing quote
    let result = character(cursor);
}

