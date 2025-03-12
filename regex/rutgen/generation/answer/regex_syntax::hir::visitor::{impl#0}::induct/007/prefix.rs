// Answer 0

#[test]
fn test_induct_repetition_min_zero() {
    let repetition = Repetition { min: 0, max: None, greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition.clone()), props: Properties {} };
    let mut visitor = HeapVisitor::new();
    let result = visitor.induct(&hir);
    let expected = Some(Frame::Repetition(&repetition));
    std::mem::discriminant(&result) == std::mem::discriminant(&expected);
}

#[test]
fn test_induct_repetition_min_five() {
    let repetition = Repetition { min: 5, max: Some(5), greedy: false, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition.clone()), props: Properties {} };
    let mut visitor = HeapVisitor::new();
    let result = visitor.induct(&hir);
    let expected = Some(Frame::Repetition(&repetition));
    std::mem::discriminant(&result) == std::mem::discriminant(&expected);
}

#[test]
fn test_induct_repetition_min_four_max_none() {
    let repetition = Repetition { min: 4, max: None, greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition.clone()), props: Properties {} };
    let mut visitor = HeapVisitor::new();
    let result = visitor.induct(&hir);
    let expected = Some(Frame::Repetition(&repetition));
    std::mem::discriminant(&result) == std::mem::discriminant(&expected);
}

#[test]
fn test_induct_repetition_min_two_max_five() {
    let repetition = Repetition { min: 2, max: Some(5), greedy: false, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition.clone()), props: Properties {} };
    let mut visitor = HeapVisitor::new();
    let result = visitor.induct(&hir);
    let expected = Some(Frame::Repetition(&repetition));
    std::mem::discriminant(&result) == std::mem::discriminant(&expected);
}

#[test]
fn test_induct_repetition_empty_sub() {
    let repetition = Repetition { min: 3, max: Some(7), greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition.clone()), props: Properties {} };
    let mut visitor = HeapVisitor::new();
    let result = visitor.induct(&hir);
    let expected = Some(Frame::Repetition(&repetition));
    std::mem::discriminant(&result) == std::mem::discriminant(&expected);
}

