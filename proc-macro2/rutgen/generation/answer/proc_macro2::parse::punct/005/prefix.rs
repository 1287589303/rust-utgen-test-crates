// Answer 0

#[test]
fn test_punct_single_quote_valid() {
    let cursor = Cursor { rest: "'abc" };
    let result = punct(cursor);
}

#[test]
fn test_punct_single_quote_invalid_identifier() {
    let cursor = Cursor { rest: "'ident" };
    let result = punct(cursor);
}

