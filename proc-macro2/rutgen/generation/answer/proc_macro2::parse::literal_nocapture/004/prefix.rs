// Answer 0

#[test]
fn test_literal_nocapture_string() {
    let input = Cursor { rest: "\"valid string\" rest" };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_byte_string() {
    let input = Cursor { rest: "b\"valid byte string\" rest" };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_c_string() {
    let input = Cursor { rest: "c\"valid c string\" rest" };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_byte() {
    let input = Cursor { rest: "b'c' rest" };
    let _ = literal_nocapture(input);
}

