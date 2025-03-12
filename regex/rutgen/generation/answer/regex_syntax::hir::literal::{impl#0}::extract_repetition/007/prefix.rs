// Answer 0

#[test]
fn test_extract_repetition_case_1() {
    let rep = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: true,
        sub: Box::new(hir::Hir::Literal(hir::Literal(vec![b'a']))),
    };
    let extractor = Extractor::new().limit_repeat(1);
    let result = extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case_2() {
    let rep = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: false,
        sub: Box::new(hir::Hir::Literal(hir::Literal(vec![b'b']))),
    };
    let extractor = Extractor::new().limit_repeat(1);
    let result = extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case_3() {
    let rep = hir::Repetition {
        min: 1,
        max: Some(1),
        greedy: true,
        sub: Box::new(hir::Hir::Literal(hir::Literal(vec![b'c']))),
    };
    let extractor = Extractor::new().limit_repeat(1);
    let result = extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case_4() {
    let rep = hir::Repetition {
        min: 2,
        max: Some(2),
        greedy: false,
        sub: Box::new(hir::Hir::Literal(hir::Literal(vec![b'd']))),
    };
    let extractor = Extractor::new().limit_repeat(2);
    let result = extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_case_5() {
    let rep = hir::Repetition {
        min: 3,
        max: Some(3),
        greedy: true,
        sub: Box::new(hir::Hir::Literal(hir::Literal(vec![b'e']))),
    };
    let extractor = Extractor::new().limit_repeat(3);
    let result = extractor.extract_repetition(&rep);
}

