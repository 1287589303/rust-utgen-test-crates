// Answer 0

#[test]
fn test_take_until_newline_or_eof_empty_input() {
    let cursor = Cursor { rest: "" };
    take_until_newline_or_eof(cursor);
}

#[test]
fn test_take_until_newline_or_eof_single_non_newline_char() {
    let cursor = Cursor { rest: "a" };
    take_until_newline_or_eof(cursor);
}

#[test]
fn test_take_until_newline_or_eof_two_non_newline_chars() {
    let cursor = Cursor { rest: "ab" };
    take_until_newline_or_eof(cursor);
}

#[test]
fn test_take_until_newline_or_eof_long_string_without_newlines() {
    let cursor = Cursor { rest: "this is a test string without newlines" };
    take_until_newline_or_eof(cursor);
}

#[test]
fn test_take_until_newline_or_eof_single_space() {
    let cursor = Cursor { rest: " " };
    take_until_newline_or_eof(cursor);
}

#[test]
fn test_take_until_newline_or_eof_single_tab() {
    let cursor = Cursor { rest: "\t" };
    take_until_newline_or_eof(cursor);
}

#[test]
fn test_take_until_newline_or_eof_with_special_characters() {
    let cursor = Cursor { rest: "!@#$%^&*()" };
    take_until_newline_or_eof(cursor);
}

