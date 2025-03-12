// Answer 0

#[test]
fn test_flatten_capture_empty() {
    let empty_capture = Hir::capture(hir::Capture { sub: Hir::empty(), ..Default::default() }); 
    let result = flatten(&empty_capture);
}

#[test]
fn test_flatten_capture_literal() {
    let literal_capture = Hir::capture(hir::Capture { sub: Hir::literal(literal::Literal::new("test".into())), ..Default::default() }); 
    let result = flatten(&literal_capture);
}

#[test]
fn test_flatten_capture_alternation() {
    let alternation_capture = Hir::capture(hir::Capture { sub: Hir::alternation(vec![Hir::literal(literal::Literal::new("a".into())), Hir::literal(literal::Literal::new("b".into()))]), ..Default::default() }); 
    let result = flatten(&alternation_capture);
}

#[test]
fn test_flatten_capture_repetition() {
    let repetition_capture = Hir::capture(hir::Capture { sub: Hir::repetition(hir::Repetition::new(Hir::literal(literal::Literal::new("abc".into())), None)), ..Default::default() }); 
    let result = flatten(&repetition_capture);
}

