// Answer 0

#[test]
fn test_punct_with_ident_starting_with_single_quote() {
    let cursor = Cursor {
        rest: "$'ident",
        off: 0,
    };
    let result = punct(cursor);
}

#[test]
fn test_punct_with_ident_starting_with_single_quote_after_space() {
    let cursor = Cursor {
        rest: "$ 'ident",
        off: 0,
    };
    let result = punct(cursor);
}

#[test]
fn test_punct_with_ident_starting_with_single_quote_and_multiple_identifiers() {
    let cursor = Cursor {
        rest: "$'ident1 'ident2",
        off: 0,
    };
    let result = punct(cursor);
}

#[test]
fn test_punct_with_ident_starting_with_single_quote_after_other_punctuations() {
    let cursor = Cursor {
        rest: "$; 'ident",
        off: 0,
    };
    let result = punct(cursor);
}

