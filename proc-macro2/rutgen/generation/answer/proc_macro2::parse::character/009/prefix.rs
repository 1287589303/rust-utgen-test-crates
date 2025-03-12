// Answer 0

#[test]
fn test_character_with_backslash_n() {
    let input = Cursor { rest: "'\\n" };
    let _ = character(input);
}

#[test]
fn test_character_with_backslash_x() {
    let input = Cursor { rest: "'\\x" };
    let _ = character(input);
}

#[test]
fn test_character_with_backslash_u() {
    let input = Cursor { rest: "'\\u" };
    let _ = character(input);
}

