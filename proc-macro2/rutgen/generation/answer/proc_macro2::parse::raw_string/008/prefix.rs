// Answer 0

#[test]
fn test_raw_string_with_delimiter_and_no_quotes() {
    let input_rest = "#" + &"a".repeat(256); // 256 characters after '#'
    let input = Cursor { rest: &input_rest, off: 0 };
    let result = raw_string(input);
}

#[test]
fn test_raw_string_with_delimiter_and_cr_without_newline() {
    let input_rest = "#" + &"a".repeat(255) + "\r" + "extra_byte"; // ends with CR and an extra byte
    let input = Cursor { rest: &input_rest, off: 0 };
    let result = raw_string(input);
}

#[test]
fn test_raw_string_with_delimiter_and_cr_after_delimiter() {
    let input_rest = "#" + &"a".repeat(255) + "\r\n"; // ends with CR and LF
    let input = Cursor { rest: &input_rest, off: 0 };
    let result = raw_string(input);
}

