// Answer 0

#[test]
fn test_cooked_string_with_valid_double_quoted_string() {
    let input = Cursor { rest: "\"some text\"", off: 0 };
    let result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_empty_double_quoted_string() {
    let input = Cursor { rest: "\"\"", off: 0 };
    let result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_double_quoted_string_with_escape() {
    let input = Cursor { rest: "\"some \\\"escaped\\\" text\"", off: 0 };
    let result = cooked_string(input);
}

