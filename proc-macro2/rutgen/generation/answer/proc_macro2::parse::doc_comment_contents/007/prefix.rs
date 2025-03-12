// Answer 0

#[test]
fn test_doc_comment_contents_block_comment_no_closing() {
    let input = Cursor {
        rest: "/** comment text".into(),
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(input);
}

#[test]
fn test_doc_comment_contents_block_comment_no_closing_with_extra_chars() {
    let input = Cursor {
        rest: "/** comment text not closing ".into(),
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(input);
}

#[test]
fn test_doc_comment_contents_block_comment_with_newline_in_between() {
    let input = Cursor {
        rest: "/** comment text\nstill part of comment".into(),
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(input);
}

#[test]
fn test_doc_comment_contents_valid_start_with_no_stars() {
    let input = Cursor {
        rest: "/** this is a valid start of a comment with no stars after".into(),
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(input);
}

