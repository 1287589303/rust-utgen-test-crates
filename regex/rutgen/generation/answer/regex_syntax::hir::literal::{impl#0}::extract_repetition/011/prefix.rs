// Answer 0

#[test]
fn test_extract_repetition_case_1() {
    let extractor = Extractor::new()
        .limit_repeat(5)
        .limit_total(10);
    let subseq = Seq::singleton(Literal::exact(vec![b'a']));
    let rep = hir::Repetition {
        min: 2,
        max: Some(3),
        greedy: true,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'a'])), props: Properties::default() }),
    };
    
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case_2() {
    let extractor = Extractor::new()
        .limit_repeat(5)
        .limit_total(10);
    let subseq = Seq::singleton(Literal::exact(vec![b'b']));
    let rep = hir::Repetition {
        min: 3,
        max: Some(3),
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'b'])), props: Properties::default() }),
    };
    
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case_3() {
    let extractor = Extractor::new()
        .limit_repeat(5)
        .limit_total(10);
    let subseq = Seq::singleton(Literal::exact(vec![b'c']));
    let rep = hir::Repetition {
        min: 1,
        max: Some(5),
        greedy: true,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'c'])), props: Properties::default() }),
    };
    
    extractor.extract_repetition(&rep);
}

