// Answer 0

#[test]
fn test_doc_comment_contents_normal_text() {
    let cursor = Cursor { rest: "Normal text" };
    let _result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_empty_string() {
    let cursor = Cursor { rest: "" };
    let _result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_no_comment() {
    let cursor = Cursor { rest: "Just some random text without comment marks." };
    let _result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_leading_spaces() {
    let cursor = Cursor { rest: "    Indented text" };
    let _result = doc_comment_contents(cursor);
}

