// Answer 0

#[test]
fn test_punct_char_with_double_slash() {
    let input = Cursor { rest: "//example" };
    let result = punct_char(input);
}

#[test]
fn test_punct_char_with_block_comment() {
    let input = Cursor { rest: "/*example" };
    let result = punct_char(input);
}

#[test]
fn test_punct_char_with_empty_string() {
    let input = Cursor { rest: "" };
    let result = punct_char(input);
}

