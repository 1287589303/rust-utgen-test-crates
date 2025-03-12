// Answer 0

#[test]
fn test_block_comment_nested_comments() {
    let cursor = Cursor { rest: "/* comment1 /* comment2 */ comment3 */" };
    let result = block_comment(cursor);
}

#[test]
fn test_block_comment_unbalanced_comments() {
    let cursor = Cursor { rest: "/* comment1 /* comment2 */ comment3" };
    let result = block_comment(cursor);
}

#[test]
fn test_block_comment_single_depth() {
    let cursor = Cursor { rest: "/* comment1 */" };
    let result = block_comment(cursor);
}

#[test]
fn test_block_comment_multiple_nested() {
    let cursor = Cursor { rest: "/* /* /* nested comment */ */ */" };
    let result = block_comment(cursor);
}

#[test]
fn test_block_comment_no_closing() {
    let cursor = Cursor { rest: "/* unclosed comment" };
    let result = block_comment(cursor);
}

