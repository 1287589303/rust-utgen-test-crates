// Answer 0

#[test]
fn test_literal_nocapture_valid_string() {
    let cursor = Cursor { rest: "\"valid string\"" };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_valid_raw_string() {
    let cursor = Cursor { rest: "r\"raw string\"" };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_invalid_c_string() {
    let cursor = Cursor { rest: "c\"invalid string\"" };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_valid_byte_string() {
    let cursor = Cursor { rest: "b\"byte string\"" };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_valid_raw_byte_string() {
    let cursor = Cursor { rest: "br\"raw byte string\"" };
    let _ = literal_nocapture(cursor);
}

