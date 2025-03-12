// Answer 0

#[test]
fn test_extract_repetition_min_gt_0_max_some_1_greedy_true() {
    let mut extractor = Extractor::new().limit_repeat(5);
    let rep = Repetition {
        min: 3,
        max: Some(1),
        greedy: true,
        sub: Box::new(Hir { kind: HirKind::Literal(Literal { bytes: vec![b'a'], exact: true }), props: Properties {} }),
    };
    let hir = Hir { kind: HirKind::Repetition(rep), props: Properties {} };
    extractor.extract(&hir);
}

#[test]
fn test_extract_repetition_min_gt_0_max_some_1_greedy_false() {
    let mut extractor = Extractor::new().limit_repeat(5);
    let rep = Repetition {
        min: 2,
        max: Some(1),
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Literal(Literal { bytes: vec![b'b'], exact: true }), props: Properties {} }),
    };
    let hir = Hir { kind: HirKind::Repetition(rep), props: Properties {} };
    extractor.extract(&hir);
}

#[test]
fn test_extract_repetition_min_gt_0_max_none_greedy_true() {
    let mut extractor = Extractor::new().limit_repeat(5);
    let rep = Repetition {
        min: 1,
        max: None,
        greedy: true,
        sub: Box::new(Hir { kind: HirKind::Literal(Literal { bytes: vec![b'c'], exact: true }), props: Properties {} }),
    };
    let hir = Hir { kind: HirKind::Repetition(rep), props: Properties {} };
    extractor.extract(&hir);
}

#[test]
fn test_extract_repetition_min_gt_0_max_none_greedy_false() {
    let mut extractor = Extractor::new().limit_repeat(5);
    let rep = Repetition {
        min: 4,
        max: None,
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Literal(Literal { bytes: vec![b'd'], exact: true }), props: Properties {} }),
    };
    let hir = Hir { kind: HirKind::Repetition(rep), props: Properties {} };
    extractor.extract(&hir);
}

