// Answer 0

#[test]
fn test_string_with_cooked_string() {
    let cursor = Cursor { rest: "\"test\"", off: 0 };
    let _ = string(cursor);
}

#[test]
fn test_string_with_raw_string() {
    let cursor = Cursor { rest: "r\"test\"", off: 0 };
    let _ = string(cursor);
}

#[test]
fn test_string_with_empty_cursor() {
    let cursor = Cursor { rest: "", off: 0 };
    let _ = string(cursor);
}

