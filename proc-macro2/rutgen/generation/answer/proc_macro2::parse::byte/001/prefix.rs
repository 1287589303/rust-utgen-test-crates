// Answer 0

#[test]
fn test_byte_invalid_starting_with_bapostrophe() {
    let cursor = Cursor { rest: "b'" };
    let _result = byte(cursor);
}

#[test]
fn test_byte_invalid_byte_character_backslash_z() {
    let cursor = Cursor { rest: "b'\\z'" };
    let _result = byte(cursor);
}

#[test]
fn test_byte_invalid_byte_character_backslash_xG() {
    let cursor = Cursor { rest: "b'\\xG'" };
    let _result = byte(cursor);
}

#[test]
fn test_byte_missing_closing_apostrophe() {
    let cursor = Cursor { rest: "b'\\n" };
    let _result = byte(cursor);
}

