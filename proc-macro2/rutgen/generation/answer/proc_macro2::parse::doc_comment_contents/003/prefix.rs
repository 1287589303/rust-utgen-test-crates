// Answer 0

#[test]
fn test_doc_comment_contents_block_comment() {
    let input_str = "/*! This is a block comment */";
    let cursor = Cursor {
        rest: input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_multiple_lines_block_comment() {
    let input_str = "/*! This is a block comment\n spanning multiple lines */";
    let cursor = Cursor {
        rest: input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_block_comment_with_special_chars() {
    let input_str = "/*! Block comment with special chars: @#$%^&*() */";
    let cursor = Cursor {
        rest: input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(cursor);
}

