// Answer 0

#[test]
fn test_extract_repetition_case1() {
    let mut extractor = Extractor::new();
    extractor.limit_repeat(0);
    extractor.limit_total(10);

    let rep = hir::Repetition {
        min: 0,
        max: None,
        greedy: true,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'a'])), props: Properties::default() }),
    };
    
    let result = extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case2() {
    let mut extractor = Extractor::new();
    extractor.limit_repeat(0);
    extractor.limit_total(10);

    let rep = hir::Repetition {
        min: 1,
        max: Some(2),
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'b'])), props: Properties::default() }),
    };

    let result = extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case3() {
    let mut extractor = Extractor::new();
    extractor.limit_repeat(10);
    extractor.limit_total(5);

    let rep = hir::Repetition {
        min: 10,
        max: Some(20),
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'c'])), props: Properties::default() }),
    };

    let result = extractor.extract_repetition(&rep);
}

