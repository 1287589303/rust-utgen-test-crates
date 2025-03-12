// Answer 0

#[test]
fn test_leaf_token_with_valid_literal() {
    let cursor = Cursor {
        rest: "123,",
        off: 0,
    };
    let result = leaf_token(cursor);
}

#[test]
fn test_leaf_token_with_valid_punct() {
    let cursor = Cursor {
        rest: "+error",
        off: 0,
    };
    let result = leaf_token(cursor);
}

#[test]
fn test_leaf_token_with_invalid_ident() {
    let cursor = Cursor {
        rest: "r#hello",
        off: 0,
    };
    let result = leaf_token(cursor);
}

#[test]
fn test_leaf_token_with_error_prefix() {
    let cursor = Cursor {
        rest: "(/*ERROR*/)",
        off: 0,
    };
    let result = leaf_token(cursor);
}

