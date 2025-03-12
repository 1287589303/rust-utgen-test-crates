// Answer 0

#[test]
fn test_cooked_string_invalid_backslash_seq() {
    let input = Cursor { rest: "\\a" };
    let result = cooked_string(input);
}

#[test]
fn test_cooked_string_invalid_backslash_seq_with_non_specified_char() {
    let input = Cursor { rest: "\\%" };
    let result = cooked_string(input);
}

#[test]
fn test_cooked_string_invalid_backslash_seq_with_empty_following() {
    let input = Cursor { rest: "\\" };
    let result = cooked_string(input);
}

