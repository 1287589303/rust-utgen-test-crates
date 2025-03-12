// Answer 0

#[test]
fn test_skip_whitespace_with_line_comment() {
    let cursor = Cursor { rest: "// This is a line comment" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_block_comment() {
    let cursor = Cursor { rest: "/* This is a block comment */ some code" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_mixed_whitespace() {
    let cursor = Cursor { rest: "    // Line comment\n    /* Block comment */\n   code" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_only_whitespace() {
    let cursor = Cursor { rest: "    \t  \n\r " };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_empty_string() {
    let cursor = Cursor { rest: "" };
    let result = skip_whitespace(cursor);
}

