// Answer 0

#[test]
fn test_string_with_raw_string_start() {
    let cursor = Cursor { rest: "rThis is a raw string" };
    let _ = string(cursor);
}

#[test]
fn test_string_with_random_string() {
    let cursor = Cursor { rest: "random string without quotes" };
    let _ = string(cursor);
}

#[test]
fn test_string_with_non_quoted_input() {
    let cursor = Cursor { rest: "input not starting with r or \"" };
    let _ = string(cursor);
}

