// Answer 0

#[test]
fn test_literal_nocapture_string_valid() {
    let input = Cursor { rest: "\"test\"", off: 0 };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_string_with_escape() {
    let input = Cursor { rest: "\"test\\n\"", off: 0 };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_string_with_escaped_quote() {
    let input = Cursor { rest: "\"test \\\"quoted\\\"\"", off: 0 };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_empty_string() {
    let input = Cursor { rest: "\"\"", off: 0 };
    let _ = literal_nocapture(input);
}

#[test]
fn test_literal_nocapture_string_no_byte_prefix() {
    let input = Cursor { rest: "\"not a byte string\"", off: 0 };
    let _ = literal_nocapture(input);
}

