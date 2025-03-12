// Answer 0

#[test]
fn test_extract_repetition_case1() {
    let extractor = Extractor::new().kind(ExtractKind::Prefix);
    let repetition = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: false,
        sub: Box::new(Hir {
            kind: hir::HirKind::Literal(hir::Literal(vec![b'a'])),
            props: Properties::default(),
        }),
    };
    extractor.extract_repetition(&repetition);
}

#[test]
fn test_extract_repetition_case2() {
    let extractor = Extractor::new().kind(ExtractKind::Suffix);
    let repetition = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: false,
        sub: Box::new(Hir {
            kind: hir::HirKind::Literal(hir::Literal(vec![b'b'])),
            props: Properties::default(),
        }),
    };
    extractor.extract_repetition(&repetition);
}

#[test]
fn test_extract_repetition_case3() {
    let extractor = Extractor::new().kind(ExtractKind::Prefix);
    let repetition = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: false,
        sub: Box::new(Hir {
            kind: hir::HirKind::Literal(hir::Literal(vec![b'c'])),
            props: Properties::default(),
        }),
    };
    extractor.extract_repetition(&repetition);
}

#[test]
fn test_extract_repetition_case4() {
    let extractor = Extractor::new().kind(ExtractKind::Suffix);
    let repetition = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: false,
        sub: Box::new(Hir {
            kind: hir::HirKind::Literal(hir::Literal(vec![b'd'])),
            props: Properties::default(),
        }),
    };
    extractor.extract_repetition(&repetition);
}

