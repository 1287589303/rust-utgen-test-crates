// Answer 0

#[test]
fn test_punct_char_with_comment_start() {
    let cursor = Cursor { rest: "/* This is a comment" };
    let _result = punct_char(cursor);
}

#[test]
fn test_punct_char_with_comment_start_and_extra_chars() {
    let cursor = Cursor { rest: "/* Comment with extra chars" };
    let _result = punct_char(cursor);
}

#[test]
fn test_punct_char_with_non_punct_char_after_comment_start() {
    let cursor = Cursor { rest: "/*A non-punctuation character" };
    let _result = punct_char(cursor);
}

