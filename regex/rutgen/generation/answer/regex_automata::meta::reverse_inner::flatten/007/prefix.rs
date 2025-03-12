// Answer 0

#[test]
fn test_flatten_literal_ascii() {
    let ascii_literal = Hir::literal(literal::Literal::new("test".to_string()));
    let result = flatten(&ascii_literal);
}

#[test]
fn test_flatten_literal_utf8() {
    let utf8_literal = Hir::literal(literal::Literal::new("你好".to_string()));
    let result = flatten(&utf8_literal);
}

#[test]
fn test_flatten_literal_empty() {
    let empty_literal = Hir::literal(literal::Literal::new("".to_string()));
    let result = flatten(&empty_literal);
}

#[test]
fn test_flatten_literal_large() {
    let large_literal = Hir::literal(literal::Literal::new("a".repeat(1000)));
    let result = flatten(&large_literal);
}

