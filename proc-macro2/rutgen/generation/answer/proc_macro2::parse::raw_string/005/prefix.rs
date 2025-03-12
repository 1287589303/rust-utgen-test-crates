// Answer 0

#[test]
fn test_raw_string_with_r_followed_by_n_and_exceeding_length() {
    let input_str = "a".repeat(255) + "\r\nx";
    let cursor = Cursor { rest: &input_str };

    let result = raw_string(cursor);
}

#[test]
fn test_raw_string_with_r_followed_by_n_and_no_more_bytes() {
    let input_str = "a".repeat(255) + "\r\n";
    let cursor = Cursor { rest: &input_str };

    let result = raw_string(cursor);
}

#[test]
fn test_raw_string_with_r_followed_by_n_and_non_newline_after() {
    let input_str = "a".repeat(255) + "\r\nx";
    let cursor = Cursor { rest: &input_str };

    let result = raw_string(cursor);
}

#[test]
fn test_raw_string_with_multiple_r_before_n() {
    let input_str = "a".repeat(255) + "\r\r\nx";
    let cursor = Cursor { rest: &input_str };

    let result = raw_string(cursor);
}

