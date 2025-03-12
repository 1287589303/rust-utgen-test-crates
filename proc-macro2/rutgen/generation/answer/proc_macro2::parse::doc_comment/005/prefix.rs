// Answer 0

#[test]
fn test_doc_comment_valid_case() {
    let input = Cursor {
        rest: "//! This is a comment\r\nNext line".as_ref(),
        off: 0,
    };
    let mut trees = TokenStreamBuilder::new();
    let result = doc_comment(input, &mut trees);
}

#[test]
fn test_doc_comment_with_bare_cr_and_newline() {
    let input = Cursor {
        rest: "//! Comment before bare CR\r\nSecond line".as_ref(),
        off: 0,
    };
    let mut trees = TokenStreamBuilder::new();
    let result = doc_comment(input, &mut trees);
}

