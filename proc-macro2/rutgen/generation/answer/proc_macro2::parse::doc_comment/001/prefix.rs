// Answer 0

#[test]
fn test_doc_comment_err_while_starting_with_bang() {
    let input = Cursor { rest: "//! invalid comment", off: 0 };
    let mut trees = TokenStreamBuilder::new();
    let _ = doc_comment(input, &mut trees);
}

#[test]
fn test_doc_comment_err_while_starting_with_block() {
    let input = Cursor { rest: "/*! invalid block comment */", off: 0 };
    let mut trees = TokenStreamBuilder::new();
    let _ = doc_comment(input, &mut trees);
}

#[test]
fn test_doc_comment_err_while_starting_with_slash() {
    let input = Cursor { rest: "/// invalid", off: 0 };
    let mut trees = TokenStreamBuilder::new();
    let _ = doc_comment(input, &mut trees);
}

#[test]
fn test_doc_comment_err_while_starting_with_block_multiline() {
    let input = Cursor { rest: "/** invalid block comment without closing", off: 0 };
    let mut trees = TokenStreamBuilder::new();
    let _ = doc_comment(input, &mut trees);
}

#[test]
fn test_doc_comment_err_bare_cr_newline() {
    let input = Cursor { rest: "//! Comment\r without newline", off: 0 };
    let mut trees = TokenStreamBuilder::new();
    let _ = doc_comment(input, &mut trees);
}

#[test]
fn test_doc_comment_err_incorrect_escape_sequence() {
    let input = Cursor { rest: "/// This comment has an invalid escape sequence \\\n", off: 0 };
    let mut trees = TokenStreamBuilder::new();
    let _ = doc_comment(input, &mut trees);
}

