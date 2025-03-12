// Answer 0

#[test]
fn test_literal_nocapture_string() {
    let cursor = Cursor { rest: "\"hello\"".to_string().as_str() };
    let result = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_byte_string() {
    let cursor = Cursor { rest: "b\"hello\"".to_string().as_str() };
    let result = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_c_string() {
    let cursor = Cursor { rest: "c\"hello\"".to_string().as_str() };
    let result = literal_nocapture(cursor);
}

