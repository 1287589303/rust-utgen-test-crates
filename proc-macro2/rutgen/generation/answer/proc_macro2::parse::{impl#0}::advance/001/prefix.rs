// Answer 0

#[test]
fn test_advance_zero_bytes() {
    let cursor = Cursor { rest: "test", off: 0 };
    let new_cursor = cursor.advance(0);
}

#[test]
fn test_advance_one_byte() {
    let cursor = Cursor { rest: "test", off: 0 };
    let new_cursor = cursor.advance(1);
}

#[test]
fn test_advance_to_length() {
    let cursor = Cursor { rest: "test", off: 0 };
    let new_cursor = cursor.advance(4);
}

#[test]
fn test_advance_more_than_length() {
    let cursor = Cursor { rest: "test", off: 0 };
    let new_cursor = cursor.advance(5);
}

#[test]
fn test_advance_with_non_zero_offset() {
    let cursor = Cursor { rest: "sample", off: 5 };
    let new_cursor = cursor.advance(3);
}

#[test]
fn test_advance_empty_string() {
    let cursor = Cursor { rest: "", off: 0 };
    let new_cursor = cursor.advance(0);
}

