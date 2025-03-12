// Answer 0

#[test]
fn test_top_concat_with_empty() {
    use regex_syntax::hir::{Hir, HirKind};
    let hir = Hir::empty();
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_with_literal() {
    use regex_syntax::hir::{Hir, HirKind, literal};
    let hir = Hir::literal(literal::Literal::new("test".into()));
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_with_class() {
    use regex_syntax::hir::{Hir, HirKind, Class};
    let class = Class::new(vec![], true);
    let hir = Hir::class(class);
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_with_repetition() {
    use regex_syntax::hir::{Hir, HirKind, Repetition};
    let hir = Hir::repetition(Box::new(Hir::literal(literal::Literal::new("test".into()))), Repetition::one());
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_with_look() {
    use regex_syntax::hir::{Hir, HirKind, Look};
    let hir = Hir::look(Box::new(Hir::literal(literal::Literal::new("test".into()))));
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_with_alternation() {
    use regex_syntax::hir::{Hir, HirKind, Alternation};
    let hir = Hir::alternation(vec![Hir::literal(literal::Literal::new("a".into())), Hir::literal(literal::Literal::new("b".into()))]);
    let result = top_concat(&hir);
}

