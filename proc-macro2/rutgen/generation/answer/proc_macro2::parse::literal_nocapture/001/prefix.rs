// Answer 0

#[test]
fn test_literal_nocapture_string_rejection() {
    let cursor = Cursor { rest: "\"invalid\" extra", off: 0 };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_byte_string_rejection() {
    let cursor = Cursor { rest: "b\"invalid\" extra", off: 0 };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_c_string_rejection() {
    let cursor = Cursor { rest: "c\"invalid\" extra", off: 0 };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_byte_rejection() {
    let cursor = Cursor { rest: "b'invalid' extra", off: 0 };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_character_rejection() {
    let cursor = Cursor { rest: "'invalid' extra", off: 0 };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_float_rejection() {
    let cursor = Cursor { rest: "3.14invalid extra", off: 0 };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_int_rejection() {
    let cursor = Cursor { rest: "123invalid extra", off: 0 };
    let _ = literal_nocapture(cursor);
}

