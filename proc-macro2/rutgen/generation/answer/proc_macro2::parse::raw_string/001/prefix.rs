// Answer 0

#[test]
fn test_raw_string_empty_cursor() {
    let cursor = Cursor { rest: "" };
    let _ = raw_string(cursor);
}

#[test]
fn test_raw_string_single_char_no_delimiter() {
    let cursor = Cursor { rest: "a" };
    let _ = raw_string(cursor);
}

#[test]
fn test_raw_string_delimiter_beyond_256_bytes() {
    let long_string = std::iter::repeat('x').take(260).collect::<String>() + "###";
    let cursor = Cursor { rest: &long_string };
    let _ = raw_string(cursor);
}

#[test]
fn test_raw_string_valid_string_with_delimiter() {
    let long_string = std::iter::repeat('x').take(256).collect::<String>() + "###";
    let cursor = Cursor { rest: &long_string };
    let _ = raw_string(cursor);
}

