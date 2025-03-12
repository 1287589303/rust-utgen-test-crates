// Answer 0

#[test]
fn test_raw_byte_string_with_invalid_return_character() {
    let cursor = Cursor { rest: "#some raw string with a return character \r and some other text\"".into() };
    let result = raw_byte_string(cursor);
}

#[test]
fn test_raw_byte_string_with_valid_return_and_subsequent_text() {
    let cursor = Cursor { rest: "#another raw string before the delimiter \rnon-newline text\"".into() };
    let result = raw_byte_string(cursor);
}

#[test]
fn test_raw_byte_string_with_non_ascii_after_return() {
    let cursor = Cursor { rest: "#yet another raw string with return \rθ and then text\"".into() };
    let result = raw_byte_string(cursor);
}

#[test]
fn test_raw_byte_string_with_shorter_length() {
    let cursor = Cursor { rest: "#short \rstring that does not end with newline\"".into() };
    let result = raw_byte_string(cursor);
}

