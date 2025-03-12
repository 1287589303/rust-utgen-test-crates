// Answer 0

#[test]
fn test_leaf_token_literal() {
    let cursor = Cursor {
        rest: r#""valid literal""#,
        #[cfg(span_locations)]
        off: 0,
    };
    let result = leaf_token(cursor);
}

#[test]
fn test_leaf_token_punct() {
    let cursor = Cursor {
        rest: r#"+ valid punct"#,
        #[cfg(span_locations)]
        off: 0,
    };
    let result = leaf_token(cursor);
}

#[test]
fn test_leaf_token_ident() {
    let cursor = Cursor {
        rest: "valid_identifier",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = leaf_token(cursor);
}

#[test]
fn test_leaf_token_error() {
    let cursor = Cursor {
        rest: "(/*ERROR*/) additional text",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = leaf_token(cursor);
}

