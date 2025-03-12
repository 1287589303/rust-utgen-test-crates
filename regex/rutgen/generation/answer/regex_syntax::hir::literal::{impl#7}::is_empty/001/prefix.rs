// Answer 0

#[test]
fn test_literal_is_empty_with_empty_bytes() {
    let literal = Literal::exact(vec![]);
    let result = literal.is_empty();
}

#[test]
fn test_literal_is_empty_with_non_empty_bytes() {
    let literal = Literal::exact(vec![1, 2, 3]);
    let result = literal.is_empty();
}

