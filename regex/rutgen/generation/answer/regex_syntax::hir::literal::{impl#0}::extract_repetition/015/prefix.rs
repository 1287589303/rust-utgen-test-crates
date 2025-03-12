// Answer 0

#[test]
fn test_extract_repetition_case_1() {
    let mut extractor = Extractor::new();
    extractor.limit_repeat(2);
    extractor.limit_total(5);
    
    let subseq = Seq::singleton(Literal::exact(vec![b'a']));
    let rep = hir::Repetition {
        min: 3,
        max: None,
        greedy: true,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'a'])), props: Default::default() }),
    };
    
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case_2() {
    let mut extractor = Extractor::new();
    extractor.limit_repeat(3);
    extractor.limit_total(5);
    
    let subseq = Seq::singleton(Literal::exact(vec![b'b']));
    let rep = hir::Repetition {
        min: 2,
        max: None,
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'b'])), props: Default::default() }),
    };
    
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case_3() {
    let mut extractor = Extractor::new();
    extractor.limit_repeat(4);
    extractor.limit_total(6);
    
    let subseq = Seq::singleton(Literal::exact(vec![b'c']));
    let rep = hir::Repetition {
        min: 5,
        max: None,
        greedy: true,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'c'])), props: Default::default() }),
    };
    
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case_4() {
    let mut extractor = Extractor::new();
    extractor.limit_repeat(1);
    extractor.limit_total(10);
    
    let subseq = Seq::singleton(Literal::exact(vec![b'd']));
    let rep = hir::Repetition {
        min: 1,
        max: None,
        greedy: false,
        sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![b'd'])), props: Default::default() }),
    };
    
    extractor.extract_repetition(&rep);
}

