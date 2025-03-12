// Answer 0

#[test]
fn test_skip_whitespace_single_line_comment() {
    let input = Cursor { rest: "// comment\n" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_single_line_comment_with_spaces() {
    let input = Cursor { rest: "// comment with spaces\n" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_inline_single_line_comment() {
    let input = Cursor { rest: "// comment with no trailing newline" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_single_line_comment_with_multiple_newlines() {
    let input = Cursor { rest: "// comment\n\n" };
    let result = skip_whitespace(input);
}

