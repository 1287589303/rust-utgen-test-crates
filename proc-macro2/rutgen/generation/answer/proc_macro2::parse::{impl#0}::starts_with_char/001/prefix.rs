// Answer 0

#[test]
fn test_starts_with_char_first_character() {
    let cursor = Cursor { rest: "abc" };
    cursor.starts_with_char('a');
}

#[test]
fn test_starts_with_char_middle_character() {
    let cursor = Cursor { rest: "abc" };
    cursor.starts_with_char('b');
}

#[test]
fn test_starts_with_char_last_character() {
    let cursor = Cursor { rest: "abc" };
    cursor.starts_with_char('c');
}

#[test]
fn test_starts_with_char_not_present() {
    let cursor = Cursor { rest: "abc" };
    cursor.starts_with_char('d');
}

#[test]
fn test_starts_with_char_special_character() {
    let cursor = Cursor { rest: "!@#$%^&*()" };
    cursor.starts_with_char('!');
}

#[test]
fn test_starts_with_char_whitespace() {
    let cursor = Cursor { rest: " hello" };
    cursor.starts_with_char(' ');
}

