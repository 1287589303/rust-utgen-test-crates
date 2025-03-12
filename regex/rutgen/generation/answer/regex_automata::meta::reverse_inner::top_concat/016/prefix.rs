// Answer 0

#[test]
fn test_top_concat_empty() {
    use regex_syntax::hir::{Hir, HirKind};

    let hir = Hir::empty();
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_literal() {
    use regex_syntax::hir::{Hir, HirKind, literal};

    let hir = Hir::literal(literal::Literal::new("test"));
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_class() {
    use regex_syntax::hir::{Hir, HirKind, Class};

    let hir = Hir::class(Class::new(vec!['a', 'b', 'c']));
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_look() {
    use regex_syntax::hir::{Hir, HirKind, Look};

    let hir = Hir::look(Look::new(hir::LookKind::LookAhead, Hir::literal(literal::Literal::new("test"))));
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_repetition() {
    use regex_syntax::hir::{Hir, HirKind, Repetition};

    let hir = Hir::repetition(Repetition::new(Hir::literal(literal::Literal::new("test")), 1..=3));
    let result = top_concat(&hir);
}

#[test]
fn test_top_concat_alternation() {
    use regex_syntax::hir::{Hir, HirKind};

    let hir = Hir::alternation(vec![Hir::literal(literal::Literal::new("test1")), Hir::literal(literal::Literal::new("test2"))]);
    let result = top_concat(&hir);
}

