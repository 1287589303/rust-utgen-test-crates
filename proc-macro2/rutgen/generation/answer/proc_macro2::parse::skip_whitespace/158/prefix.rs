// Answer 0

#[test]
fn test_skip_whitespace_single_line_comment() {
    let cursor = Cursor { rest: "// this is a comment\n remaining text" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_block_comment() {
    let cursor = Cursor { rest: "/* this is a block comment */ remaining text" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_inline_space() {
    let cursor = Cursor { rest: "/    some valid text" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_mixed_content() {
    let cursor = Cursor { rest: "/**/ some other text" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_escaped_comments() {
    let cursor = Cursor { rest: "/* comment /* nested */ */ text" };
    let result = skip_whitespace(cursor);
}

