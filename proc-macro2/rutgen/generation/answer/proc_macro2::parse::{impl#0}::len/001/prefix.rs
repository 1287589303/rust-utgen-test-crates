// Answer 0

#[test]
fn test_len_empty_string() {
    let cursor = Cursor { rest: "" };
    let _ = cursor.len();
}

#[test]
fn test_len_single_character() {
    let cursor = Cursor { rest: "a" };
    let _ = cursor.len();
}

#[test]
fn test_len_short_string() {
    let cursor = Cursor { rest: "hello" };
    let _ = cursor.len();
}

#[test]
fn test_len_medium_string() {
    let cursor = Cursor { rest: "this is a test string with medium length" };
    let _ = cursor.len();
}

#[test]
fn test_len_long_string() {
    let long_string = "a".repeat(1000);
    let cursor = Cursor { rest: &long_string };
    let _ = cursor.len();
}

#[test]
fn test_len_near_capacity_string() {
    let near_capacity_string = "a".repeat(usize::MAX as usize);
    let cursor = Cursor { rest: &near_capacity_string };
    let _ = cursor.len();
}

#[test]
fn test_len_large_string() {
    let large_string = "a".repeat(usize::MAX as usize - 1);
    let cursor = Cursor { rest: &large_string };
    let _ = cursor.len();
}

