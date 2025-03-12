// Answer 0

#[test]
fn test_block_comment_nested_case() {
    let input = Cursor {
        rest: "/* comment /* nested comment */ comment */",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = block_comment(input);
}

#[test]
fn test_block_comment_multiple_nested() {
    let input = Cursor {
        rest: "/* outer /* inner /* deeper */ inner end */ outer end */",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = block_comment(input);
}

#[test]
fn test_block_comment_single_nested() {
    let input = Cursor {
        rest: "/* single /* nested */ comment */",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = block_comment(input);
}

#[test]
fn test_block_comment_depth_not_zero() {
    let input = Cursor {
        rest: "/* start /* middle */ end */",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = block_comment(input);
}

