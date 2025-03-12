// Answer 0

#[test]
fn test_block_comment_unmatched_opening() {
    let cursor = Cursor {
        rest: "/* Comment /* Nested comment",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = block_comment(cursor);
}

#[test]
fn test_block_comment_unmatched_closing() {
    let cursor = Cursor {
        rest: "/* Comment /* Nested comment */ /* Another unclosed comment",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = block_comment(cursor);
}

#[test]
fn test_block_comment_only_opening() {
    let cursor = Cursor {
        rest: "/*",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = block_comment(cursor);
}

