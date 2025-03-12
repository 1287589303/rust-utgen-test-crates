// Answer 0

#[test]
fn test_canonical_binary_with_unknown_property() {
    let query = ClassQuery::Binary("unknown_property");
    let _ = query.canonical_binary("x");
}

#[test]
fn test_canonical_binary_with_unsupported_property() {
    let query = ClassQuery::Binary("unsupported_property");
    let _ = query.canonical_binary("y");
}

#[test]
fn test_canonical_binary_with_empty_property() {
    let query = ClassQuery::Binary("");
    let _ = query.canonical_binary("z");
}

