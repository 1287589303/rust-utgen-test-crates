// Answer 0

#[test]
fn test_doc_comment_with_cr_not_followed_by_nl() {
    let input = Cursor { rest: "//! Example comment\rNot followed by newline".as_ref(), off: 0 };
    let mut trees = TokenStreamBuilder::new();

    let result = doc_comment(input, &mut trees);
}

#[test]
fn test_doc_comment_with_cr_in_middle() {
    let input = Cursor { rest: "//! Comment with\rcarriage return in the middle".as_ref(), off: 0 };
    let mut trees = TokenStreamBuilder::new();

    let result = doc_comment(input, &mut trees);
}

#[test]
fn test_doc_comment_with_cr_at_end() {
    let input = Cursor { rest: "//! Comment ending with\r".as_ref(), off: 0 };
    let mut trees = TokenStreamBuilder::new();

    let result = doc_comment(input, &mut trees);
}

