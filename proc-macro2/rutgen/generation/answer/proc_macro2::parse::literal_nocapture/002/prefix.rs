// Answer 0

#[test]
fn test_literal_nocapture_string() {
    let input = Cursor { rest: "\"valid string\"", off: 0 };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_byte_string() {
    let input = Cursor { rest: "b\"valid byte string\"", off: 0 };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_c_string() {
    let input = Cursor { rest: "c\"valid C string\"", off: 0 };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_byte_character() {
    let input = Cursor { rest: "b'c'", off: 0 };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_character() {
    let input = Cursor { rest: "'c'", off: 0 };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_float() {
    let input = Cursor { rest: "3.14", off: 0 };
    let _ = literal_nocapture(input);
}

