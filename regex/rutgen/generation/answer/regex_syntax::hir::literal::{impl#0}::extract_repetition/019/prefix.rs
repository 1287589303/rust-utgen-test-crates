// Answer 0

#[test]
fn test_extract_repetition_with_greedy_repeat() {
    let grepping_limit = 10;
    let subseq = Seq::singleton(Literal::exact(vec![b'a']));

    let rep = hir::Repetition {
        min: 0,
        max: Some(2),
        greedy: true,
        sub: Box::new(subseq.clone()), // Using subseq which contains a literal 'a'
    };

    let extractor = Extractor::new().limit_repeat(grepping_limit);
    let result = extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_with_multiple_greedy() {
    let grepping_limit = 5;
    let subseq = Seq::singleton(Literal::exact(vec![b'b']));

    let rep = hir::Repetition {
        min: 0,
        max: Some(3),
        greedy: true,
        sub: Box::new(subseq.clone()), // Using subseq which contains a literal 'b'
    };

    let extractor = Extractor::new().limit_repeat(grepping_limit);
    let result = extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_with_large_greedy() {
    let grepping_limit = 20;
    let subseq = Seq::singleton(Literal::exact(vec![b'c']));

    let rep = hir::Repetition {
        min: 0,
        max: Some(4),
        greedy: true,
        sub: Box::new(subseq.clone()), // Using subseq which contains a literal 'c'
    };

    let extractor = Extractor::new().limit_repeat(grepping_limit);
    let result = extractor.extract_repetition(&rep);
}

