// Answer 0

#[test]
fn test_char_indices_empty_string() {
    let cursor = Cursor { rest: "" };
    let _ = cursor.char_indices();
}

#[test]
fn test_char_indices_single_character() {
    let cursor = Cursor { rest: "a" };
    let _ = cursor.char_indices();
}

#[test]
fn test_char_indices_multiple_characters() {
    let cursor = Cursor { rest: "hello" };
    let _ = cursor.char_indices();
}

#[test]
fn test_char_indices_special_characters() {
    let cursor = Cursor { rest: "!@#$%^&*()" };
    let _ = cursor.char_indices();
}

#[test]
fn test_char_indices_with_whitespace() {
    let cursor = Cursor { rest: "hello world" };
    let _ = cursor.char_indices();
}

#[test]
fn test_char_indices_long_string() {
    let cursor = Cursor { rest: "a".repeat(10_000) }; // maximum length test
    let _ = cursor.char_indices();
}

