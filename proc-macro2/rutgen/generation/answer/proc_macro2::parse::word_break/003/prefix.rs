// Answer 0

#[test]
fn test_word_break_empty_cursor() {
    let cursor = Cursor { rest: "" };
    let _ = word_break(cursor);
}

