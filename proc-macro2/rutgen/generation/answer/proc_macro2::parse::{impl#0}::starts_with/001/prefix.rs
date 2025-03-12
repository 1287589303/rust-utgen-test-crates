// Answer 0

#[test]
fn test_starts_with_valid_case() {
    let cursor = Cursor { rest: "hello world" };
    let result = cursor.starts_with("hello");
}

#[test]
fn test_starts_with_empty_substring() {
    let cursor = Cursor { rest: "hello world" };
    let result = cursor.starts_with("");
}

#[test]
fn test_starts_with_full_string() {
    let cursor = Cursor { rest: "hello world" };
    let result = cursor.starts_with("hello world");
}

#[test]
fn test_starts_with_non_existing_substring() {
    let cursor = Cursor { rest: "hello world" };
    let result = cursor.starts_with("world hello");
}

#[test]
fn test_starts_with_numeric() {
    let cursor = Cursor { rest: "12345" };
    let result = cursor.starts_with("123");
}

