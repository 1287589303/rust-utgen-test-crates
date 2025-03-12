// Answer 0

#[test]
fn test_block_comment_string_not_starting_with_comment() {
    let cursor = Cursor { rest: "some random text" };
    let _result = block_comment(cursor);
}

#[test]
fn test_block_comment_empty_string() {
    let cursor = Cursor { rest: "" };
    let _result = block_comment(cursor);
}

#[test]
fn test_block_comment_string_starting_without_comment() {
    let cursor = Cursor { rest: "/// this is not a comment" };
    let _result = block_comment(cursor);
}

