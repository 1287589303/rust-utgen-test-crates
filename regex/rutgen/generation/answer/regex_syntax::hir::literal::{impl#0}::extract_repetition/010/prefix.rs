// Answer 0

#[test]
fn test_extract_repetition_min_eq_max_exact() {
    let mut extractor = Extractor::new()
        .limit_repeat(1);
    
    let sub_hir = Hir {
        kind: HirKind::Literal(hir::Literal(vec![b'a'])),
        props: Properties::default(),
    };

    let rep = hir::Repetition {
        min: 1,
        max: Some(1),
        greedy: true,
        sub: Box::new(sub_hir),
    };

    let result = extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_min_eq_max_inexact() {
    let mut extractor = Extractor::new()
        .limit_repeat(1);
    
    let sub_hir = Hir {
        kind: HirKind::Literal(hir::Literal(vec![b'b'])),
        props: Properties::default(),
    };

    let rep = hir::Repetition {
        min: 1,
        max: Some(1),
        greedy: false,
        sub: Box::new(sub_hir),
    };

    let result = extractor.extract_repetition(&rep);
}

