// Answer 0

#[test]
fn test_leaf_token_with_empty_cursor() {
    let input = Cursor { rest: "" };
    let result = leaf_token(input);
}

#[test]
fn test_leaf_token_with_non_matching_prefix() {
    let input = Cursor { rest: "invalid" };
    let result = leaf_token(input);
}

#[test]
fn test_leaf_token_with_multiple_non_matching_characters() {
    let input = Cursor { rest: "123abc" };
    let result = leaf_token(input);
}

#[test]
fn test_leaf_token_with_error_string_prefix() {
    let input = Cursor { rest: "ERROR" };
    let result = leaf_token(input);
}

#[test]
fn test_leaf_token_with_valid_input_returning_error() {
    let input = Cursor { rest: "not_a_valid_literal_or_ident" };
    let result = leaf_token(input);
}

