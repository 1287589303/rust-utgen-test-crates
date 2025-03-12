// Answer 0

#[test]
fn test_block_comment_depth_increased() {
    let input = Cursor {
        rest: "/* comment /* deeper comment */",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = block_comment(input);
}

#[test]
fn test_block_comment_multiple_nesting() {
    let input = Cursor {
        rest: "/* outer /* inner /* deepest */ comment */",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = block_comment(input);
}

#[test]
fn test_block_comment_with_content_before_closing() {
    let input = Cursor {
        rest: "/* /* /* not closed */ more content",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = block_comment(input);
}

#[test]
fn test_block_comment_no_closing_tag() {
    let input = Cursor {
        rest: "/* unclosed /* nested comment",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = block_comment(input);
}

#[test]
fn test_block_comment_depth_one() {
    let input = Cursor {
        rest: "/* single depth /* still open /* more depth */",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = block_comment(input);
}

