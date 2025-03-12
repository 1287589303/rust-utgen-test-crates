// Answer 0

#[test]
fn test_flatten_empty() {
    let hir = Hir::empty();
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_literal() {
    let hir = Hir::literal(literal::Literal::new('a'));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_class() {
    let hir = Hir::class(regex_syntax::ast::Class::new(vec!['a', 'b', 'c'], false));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_look() {
    let hir = Hir::look(Hir::literal(literal::Literal::new('a')));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_repetition() {
    let sub_hir = Hir::literal(literal::Literal::new('a'));
    let hir = Hir::repetition(hir::Repetition::new(sub_hir, 0..1));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_capture() {
    let sub_hir = Hir::literal(literal::Literal::new('a'));
    let hir = Hir::capture(hir::Capture::new(sub_hir));
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_alternation() {
    let hir1 = Hir::literal(literal::Literal::new('a'));
    let hir2 = Hir::literal(literal::Literal::new('b'));
    let hir = Hir::alternation(vec![hir1, hir2]);
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_concat() {
    let hir1 = Hir::literal(literal::Literal::new('a'));
    let hir2 = Hir::literal(literal::Literal::new('b'));
    let hir = Hir::concat(vec![hir1, hir2]);
    let _result = flatten(&hir);
}

