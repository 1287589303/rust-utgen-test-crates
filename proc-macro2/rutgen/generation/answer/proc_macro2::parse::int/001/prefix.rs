// Answer 0

#[test]
fn test_int_with_empty_cursor() {
    let cursor = Cursor { rest: "" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_non_numeric_string() {
    let cursor = Cursor { rest: "abc" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_string_starting_with_non_digit() {
    let cursor = Cursor { rest: "xyz123" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_leading_zero_non_digit() {
    let cursor = Cursor { rest: "0abc" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_leading_invalid_hex() {
    let cursor = Cursor { rest: "0xgh" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_leading_invalid_octal() {
    let cursor = Cursor { rest: "0o89" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_leading_invalid_binary() {
    let cursor = Cursor { rest: "0b12" };
    let _ = int(cursor);
}

