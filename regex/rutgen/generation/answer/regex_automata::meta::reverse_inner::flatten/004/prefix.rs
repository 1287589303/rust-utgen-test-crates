// Answer 0

#[test]
fn test_flatten_repetition_with_literal() {
    let lit_hir = Hir::literal(literal::Literal::from('a'));
    let repetition_hir = Hir::repetition(hir::Repetition::new(lit_hir, 1..=3));
    let flattened_hir = flatten(&repetition_hir);
}

#[test]
fn test_flatten_repetition_with_empty() {
    let empty_hir = Hir::empty();
    let repetition_hir = Hir::repetition(hir::Repetition::new(empty_hir, 1..=3));
    let flattened_hir = flatten(&repetition_hir);
}

#[test]
fn test_flatten_repetition_with_nested_repetition() {
    let nested_repetition_hir = Hir::repetition(hir::Repetition::new(Hir::literal(literal::Literal::from('b')), 1..=2));
    let repetition_hir = Hir::repetition(hir::Repetition::new(nested_repetition_hir, 1..=3));
    let flattened_hir = flatten(&repetition_hir);
}

#[test]
fn test_flatten_repetition_with_alternation() {
    let alt_hir = Hir::alternation(vec![
        Hir::literal(literal::Literal::from('c')),
        Hir::literal(literal::Literal::from('d')),
    ]);
    let repetition_hir = Hir::repetition(hir::Repetition::new(alt_hir, 1..=3));
    let flattened_hir = flatten(&repetition_hir);
}

#[test]
fn test_flatten_repetition_with_capture() {
    let capture_hir = Hir::capture(hir::Capture { sub: Box::new(Hir::literal(literal::Literal::from('e'))), name: None });
    let repetition_hir = Hir::repetition(hir::Repetition::new(capture_hir, 1..=3));
    let flattened_hir = flatten(&repetition_hir);
}

