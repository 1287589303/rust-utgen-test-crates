// Answer 0

#[test]
fn test_flatten_look_empty() {
    let hir = Hir::look(Hir::empty());
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_look_literal() {
    let hir = Hir::look(Hir::literal(literal::Literal::from_char('a')));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_look_class() {
    let hir = Hir::look(Hir::class(vec!['a', 'b', 'c']));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_look_repetition() {
    let hir = Hir::look(Hir::repetition(hir::Repetition::zero_or_more(Hir::literal(literal::Literal::from_char('x')))));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_look_nested() {
    let inner_hir = Hir::literal(literal::Literal::from_char('b'));
    let hir = Hir::look(Hir::look(inner_hir));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_look_alternation() {
    let left = Hir::literal(literal::Literal::from_char('a'));
    let right = Hir::literal(literal::Literal::from_char('b'));
    let hir = Hir::look(Hir::alternation(vec![left, right]));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_look_concat() {
    let left = Hir::literal(literal::Literal::from_char('a'));
    let right = Hir::literal(literal::Literal::from_char('b'));
    let hir = Hir::look(Hir::concat(vec![left, right]));
    let _result = flatten(&hir);
}

