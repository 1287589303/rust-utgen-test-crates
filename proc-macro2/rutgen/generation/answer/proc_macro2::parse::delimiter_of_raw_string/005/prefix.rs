// Answer 0

#[test]
fn test_delimiter_of_raw_string_no_delimiter() {
    let input = Cursor { rest: "abcdEfghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890" };
    let result = delimiter_of_raw_string(input);
}

#[test]
fn test_delimiter_of_raw_string_exceeds_length() {
    let long_input = "x".repeat(300);
    let input = Cursor { rest: &long_input };
    let result = delimiter_of_raw_string(input);
}

#[test]
fn test_delimiter_of_raw_string_no_quotes_or_hashes() {
    let input = Cursor { rest: "this string contains no delimiters whatsoever" };
    let result = delimiter_of_raw_string(input);
}

#[test]
fn test_delimiter_of_raw_string_only_special_chars() {
    let input = Cursor { rest: "!@#$%^&*()_+=-[]{}|;:,.<>?/`~" };
    let result = delimiter_of_raw_string(input);
}

