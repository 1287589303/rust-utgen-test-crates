// Answer 0

#[test]
fn test_doc_comment_contents_valid_comment() {
    let rest = "/// This is a valid comment\n";
    let cursor = Cursor { rest };
    let result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_valid_comment_with_text() {
    let rest = "/// Another valid comment";
    let cursor = Cursor { rest };
    let result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_empty_comment() {
    let rest = "/// \n";
    let cursor = Cursor { rest };
    let result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_non_slash_after_comment() {
    let rest = "/// Content that does not start with another slash\n";
    let cursor = Cursor { rest };
    let result = doc_comment_contents(cursor);
}

