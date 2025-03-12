// Answer 0

#[test]
fn test_doc_comment_contents_edge_case_1() {
    let cursor = Cursor { rest: "/** comment with star * inside */" };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_edge_case_2() {
    let cursor = Cursor { rest: "/** another comment * more text */" };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_edge_case_3() {
    let cursor = Cursor { rest: "/**  * Just a star * here */" };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_edge_case_4() {
    let cursor = Cursor { rest: "/** sample  *  text */" };
    let _ = doc_comment_contents(cursor);
}

#[test]
fn test_doc_comment_contents_edge_case_5() {
    let cursor = Cursor { rest: "/** starting with star * and ending */" };
    let _ = doc_comment_contents(cursor);
}

