// Answer 0

#[test]
fn test_leaf_token_with_literal() {
    let initial_cursor = Cursor { rest: "42 +", off: 0 };
    let result = leaf_token(initial_cursor);
}

#[test]
fn test_leaf_token_with_different_literal() {
    let initial_cursor = Cursor { rest: "\"hello\" -", off: 0 };
    let result = leaf_token(initial_cursor);
}

#[test]
fn test_leaf_token_with_edge_case_literal() {
    let initial_cursor = Cursor { rest: "'char' *", off: 0 };
    let result = leaf_token(initial_cursor);
}

