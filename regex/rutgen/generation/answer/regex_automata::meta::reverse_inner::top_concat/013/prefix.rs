// Answer 0

#[test]
fn test_top_concat_empty() {
    let hir = Hir::empty();
    top_concat(&hir);
}

#[test]
fn test_top_concat_literal() {
    let hir = Hir::literal("test".into());
    top_concat(&hir);
}

#[test]
fn test_top_concat_class() {
    let hir = Hir::class(vec!['a', 'b', 'c']);
    top_concat(&hir);
}

#[test]
fn test_top_concat_look() {
    let hir = Hir::look(hir::Look::Assert(hir::LookAssert::Positive, Box::new(Hir::literal("look".into()))));
    top_concat(&hir);
}

#[test]
fn test_top_concat_repetition() {
    let hir = Hir::repetition(Box::new(Hir::literal("repeat".into())), 0..1, false);
    top_concat(&hir);
}

#[test]
fn test_top_concat_alternation() {
    let hir = Hir::alternation(vec![Hir::literal("alt1".into()), Hir::literal("alt2".into())]);
    top_concat(&hir);
}

#[test]
fn test_top_concat_capture_with_look() {
    let nested_hir = Hir::look(hir::Look::Assert(hir::LookAssert::Positive, Box::new(Hir::literal("nested_look".into()))));
    let hir = Hir::capture(hir::Capture { sub: Box::new(nested_hir), name: None });
    top_concat(&hir);
}

