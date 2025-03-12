// Answer 0

#[test]
fn test_flatten_empty() {
    let hir = Hir::empty();
    let result = flatten(&hir);
}

#[test]
fn test_flatten_literal() {
    let hir = Hir::literal(literal::Literal::new("test".to_string()));
    let result = flatten(&hir);
}

#[test]
fn test_flatten_class() {
    let class = hir::Class::new(vec!['a', 'b', 'c']);
    let hir = Hir::class(class);
    let result = flatten(&hir);
}

#[test]
fn test_flatten_look() {
    let look = hir::Look::new(hir::LookKind::LookBehind, Box::new(Hir::literal(literal::Literal::new("test".to_string()))));
    let hir = Hir::look(look);
    let result = flatten(&hir);
}

#[test]
fn test_flatten_repetition() {
    let repetition = hir::Repetition::new(Box::new(Hir::literal(literal::Literal::new("test".to_string()))), 1..=2);
    let hir = Hir::repetition(repetition);
    let result = flatten(&hir);
}

#[test]
fn test_flatten_capture() {
    let capture = hir::Capture {
        id: 0,
        sub: Box::new(Hir::literal(literal::Literal::new("test".to_string()))),
    };
    let hir = Hir::capture(capture);
    let result = flatten(&hir);
}

#[test]
fn test_flatten_alternation() {
    let alt1 = Hir::literal(literal::Literal::new("a".to_string()));
    let alt2 = Hir::literal(literal::Literal::new("b".to_string()));
    let hir = Hir::alternation(vec![alt1, alt2]);
    let result = flatten(&hir);
}

#[test]
fn test_flatten_concat() {
    let concat1 = Hir::literal(literal::Literal::new("a".to_string()));
    let concat2 = Hir::literal(literal::Literal::new("b".to_string()));
    let hir = Hir::concat(vec![concat1, concat2]);
    let result = flatten(&hir);
}

