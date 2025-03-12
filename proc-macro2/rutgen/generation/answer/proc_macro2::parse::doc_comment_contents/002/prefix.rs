// Answer 0

#[test]
fn test_doc_comment_contents_with_block_comment_error() {
    let input = Cursor {
        rest: "/* unclosed comment",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(input);
}

