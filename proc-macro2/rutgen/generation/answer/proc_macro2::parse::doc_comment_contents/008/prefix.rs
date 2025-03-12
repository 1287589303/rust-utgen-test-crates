// Answer 0

#[test]
fn test_doc_comment_contents_case1() {
    let input_str = "/** This is a block comment */ Next line";
    let cursor = Cursor {
        rest: input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_case2() {
    let input_str = "/** Valid comment without an asterisk after **/";
    let cursor = Cursor {
        rest: input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_case3() {
    let input_str = "/** Another valid comment */ Some text";
    let cursor = Cursor {
        rest: input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_case4() {
    let input_str = "/** Yet another comment without trailing asterisk ";
    let cursor = Cursor {
        rest: input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_case5() {
    let input_str = "/** A comment containing multiple lines\r\nthat extends over the line.";
    let cursor = Cursor {
        rest: input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = doc_comment_contents(cursor);
}

