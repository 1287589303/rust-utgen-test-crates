// Answer 0

#[test]
fn test_skip_whitespace_with_single_line_comment() {
    let cursor = Cursor { rest: "   // This is a single line comment\nint x = 0;" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_block_comment() {
    let cursor = Cursor { rest: "   /* This is a block comment */ int x = 0;" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_whitespace_and_non_comment() {
    let cursor = Cursor { rest: "    // leading whitespace\n   int x = 0;" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_mixed_comments_and_whitespace() {
    let cursor = Cursor { rest: "// Comment \n   /* Block comment starts\n   Block comment ends */ int x = 0;" };
    let result = skip_whitespace(cursor);
}

#[test]
fn test_skip_whitespace_with_multiple_whitespace() {
    let cursor = Cursor { rest: "        \t\n/* Comment with whitespace before */int y = 1;" };
    let result = skip_whitespace(cursor);
}

