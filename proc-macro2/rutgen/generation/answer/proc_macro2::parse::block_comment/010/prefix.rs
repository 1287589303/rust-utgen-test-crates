// Answer 0

#[test]
fn test_block_comment_with_unmatched_nested_comments() {
    let cursor = Cursor { rest: "/* comment /* nested comment */".to_string().as_str() };

    let result = block_comment(cursor);
}

#[test]
fn test_block_comment_with_extra_opening() {
    let cursor = Cursor { rest: "/* start /* still open".to_string().as_str() };

    let result = block_comment(cursor);
}

#[test]
fn test_block_comment_with_multiple_opening() {
    let cursor = Cursor { rest: "/* first /* second /* third".to_string().as_str() };

    let result = block_comment(cursor);
}

#[test]
fn test_block_comment_with_no_closing() {
    let cursor = Cursor { rest: "/* only opened /*".to_string().as_str() };

    let result = block_comment(cursor);
}

