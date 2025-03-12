// Answer 0

#[test]
fn test_doc_comment_contents_with_bang() {
    let cursor = Cursor { rest: "//! This is a documentation comment.\n" };
    let result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_with_bang_single_char_after() {
    let cursor = Cursor { rest: "//!A\n" };
    let result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_with_bang_multiple_chars() {
    let cursor = Cursor { rest: "//!Some more text before newline.\n" };
    let result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_with_bang_no_newline() {
    let cursor = Cursor { rest: "//!This comment goes until EOF" };
    let result = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_with_bang_edge_case() {
    let cursor = Cursor { rest: "//!\n" };
    let result = doc_comment_contents(cursor);
}

