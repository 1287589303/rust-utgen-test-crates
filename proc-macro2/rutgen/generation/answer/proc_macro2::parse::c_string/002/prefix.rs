// Answer 0

#[test]
fn test_c_string_with_cooked_string() {
    let cursor = Cursor { rest: r#"c\"Hello, World!"# };
    let _ = c_string(cursor);
}

#[test]
fn test_c_string_with_non_raw_string() {
    let cursor = Cursor { rest: r#"c\"Some text" is not raw"# };
    let _ = c_string(cursor);
}

