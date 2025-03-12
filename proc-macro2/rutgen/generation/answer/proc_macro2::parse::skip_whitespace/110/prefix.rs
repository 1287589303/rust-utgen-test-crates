// Answer 0

#[test]
fn test_skip_whitespace_with_single_line_comment() {
    let cursor = Cursor { rest: "   // comment" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_single_line_comment_with_whitespace() {
    let cursor = Cursor { rest: "   // this is a comment" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_single_line_comment_ending_whitespace() {
    let cursor = Cursor { rest: "   // comment with trailing whitespace   " };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_single_line_comment_and_characters() {
    let cursor = Cursor { rest: "   // comment followed by code" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_single_line_comment_and_multiple_lines() {
    let cursor = Cursor { rest: "   // comment\n   int x = 0;" };
    let result = skip_whitespace(cursor);
}

