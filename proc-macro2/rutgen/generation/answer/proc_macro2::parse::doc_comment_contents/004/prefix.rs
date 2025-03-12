// Answer 0

#[test]
fn test_doc_comment_contents_invalid_start() {
    let input = Cursor { rest: "/// / this is a comment" };
    let result = doc_comment_contents(input);
}

#[test]
fn test_doc_comment_contents_invalid_start_with_empty_comment() {
    let input = Cursor { rest: "/// /" };
    let result = doc_comment_contents(input);
}

#[test]
fn test_doc_comment_contents_invalid_start_with_text_after_slash() {
    let input = Cursor { rest: "/// /some text here" };
    let result = doc_comment_contents(input);
}

