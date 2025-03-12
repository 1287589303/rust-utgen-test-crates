// Answer 0

#[test]
fn test_literal_with_invalid_identifier() {
    let cursor = Cursor { rest: "abc" };
    let _ = literal(cursor);
}

#[test]
fn test_literal_with_invalid_integer() {
    let cursor = Cursor { rest: "123abc" };
    let _ = literal(cursor);
}

#[test]
fn test_literal_with_invalid_float() {
    let cursor = Cursor { rest: "3.14abc" };
    let _ = literal(cursor);
}

#[test]
fn test_literal_with_negative_float_invalid_suffix() {
    let cursor = Cursor { rest: "-3.14abc" };
    let _ = literal(cursor);
}

#[test]
fn test_literal_with_hexadecimal() {
    let cursor = Cursor { rest: "0x12" };
    let _ = literal(cursor);
}

