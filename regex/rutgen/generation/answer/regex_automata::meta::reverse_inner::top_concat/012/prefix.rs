// Answer 0

#[test]
fn test_top_concat_empty() {
    let hir = Hir::empty();
    top_concat(&hir);
}

#[test]
fn test_top_concat_literal() {
    let hir = Hir::literal("test");
    top_concat(&hir);
}

#[test]
fn test_top_concat_class() {
    let hir = Hir::class(vec!['a', 'b', 'c']);
    top_concat(&hir);
}

#[test]
fn test_top_concat_look() {
    let hir = Hir::look(hir::Look::assert(true, Hir::literal("lookahead")));
    top_concat(&hir);
}

#[test]
fn test_top_concat_repetition() {
    let hir = Hir::repetition(Box::new(Hir::literal("repeated")), hir::Repetition::zero_or_more());
    top_concat(&hir);
}

#[test]
fn test_top_concat_alternation() {
    let hir = Hir::alternation(vec![Hir::literal("option1"), Hir::literal("option2")]);
    top_concat(&hir);
}

