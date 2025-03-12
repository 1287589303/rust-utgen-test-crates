// Answer 0

#[test]
fn test_block_comment_complete() {
    let cursor = Cursor { rest: "/* comment */" };
    let _result = block_comment(cursor);
}

#[test]
fn test_block_comment_no_closing() {
    let cursor = Cursor { rest: "/* comment" };
    let _result = block_comment(cursor);
}

#[test]
fn test_block_comment_nested() {
    let cursor = Cursor { rest: "/* comment /* nested comment */ */" };
    let _result = block_comment(cursor);
}

#[test]
fn test_block_comment_unmatched_symbols() {
    let cursor = Cursor { rest: "/* comment with unmatched symbols ** /" };
    let _result = block_comment(cursor);
}

#[test]
fn test_block_comment_only_opening() {
    let cursor = Cursor { rest: "/*" };
    let _result = block_comment(cursor);
}

#[test]
fn test_block_comment_single_close() {
    let cursor = Cursor { rest: "/*/" };
    let _result = block_comment(cursor);
}

