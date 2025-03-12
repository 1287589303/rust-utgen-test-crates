// Answer 0

#[test]
fn test_extract_repetition_case1() {
    let rev = hir::Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(hir::Hir {
            kind: hir::HirKind::Literal(hir::Literal(vec![b'a'])),
            props: Default::default(),
        }),
    };
    let extractor = Extractor::new()
        .limit_repeat(5);
    extractor.extract_repetition(&rev);
}

#[test]
fn test_extract_repetition_case2() {
    let rev = hir::Repetition {
        min: 2,
        max: Some(4),
        greedy: false,
        sub: Box::new(hir::Hir {
            kind: hir::HirKind::Literal(hir::Literal(vec![b'b'])),
            props: Default::default(),
        }),
    };
    let extractor = Extractor::new()
        .limit_repeat(5);
    extractor.extract_repetition(&rev);
}

#[test]
fn test_extract_repetition_case3() {
    let rev = hir::Repetition {
        min: 1,
        max: Some(5),
        greedy: true,
        sub: Box::new(hir::Hir {
            kind: hir::HirKind::Literal(hir::Literal(vec![b'c'])),
            props: Default::default(),
        }),
    };
    let extractor = Extractor::new()
        .limit_repeat(10);
    extractor.extract_repetition(&rev);
}

#[test]
fn test_extract_repetition_case4() {
    let rev = hir::Repetition {
        min: 3,
        max: Some(6),
        greedy: false,
        sub: Box::new(hir::Hir {
            kind: hir::HirKind::Literal(hir::Literal(vec![b'd'])),
            props: Default::default(),
        }),
    };
    let extractor = Extractor::new()
        .limit_repeat(10);
    extractor.extract_repetition(&rev);
}

