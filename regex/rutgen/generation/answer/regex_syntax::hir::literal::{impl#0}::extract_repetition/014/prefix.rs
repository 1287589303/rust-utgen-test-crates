// Answer 0

#[test]
fn test_extract_repetition_min_0_max_some_not_equal_1_greedy_true() {
    let extractor = Extractor::new()
        .limit_repeat(0);
    let subseq = Seq::singleton(Literal::exact(vec![b'a']));
    let rep = hir::Repetition {
        min: 0,
        max: Some(2),
        greedy: true,
        sub: Box::new(subseq),
    };
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_min_0_max_some_equal_1_greedy_false() {
    let extractor = Extractor::new()
        .limit_repeat(0);
    let subseq = Seq::singleton(Literal::inexact(vec![b'a']));
    let rep = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: false,
        sub: Box::new(subseq),
    };
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_min_0_max_some_equal_2_greedy_true() {
    let extractor = Extractor::new()
        .limit_repeat(0);
    let subseq = Seq::singleton(Literal::inexact(vec![b'a']));
    let rep = hir::Repetition {
        min: 0,
        max: Some(2),
        greedy: true,
        sub: Box::new(subseq),
    };
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_min_0_max_some_equal_3_greedy_false() {
    let extractor = Extractor::new()
        .limit_repeat(0);
    let subseq = Seq::singleton(Literal::exact(vec![b'b']));
    let rep = hir::Repetition {
        min: 0,
        max: Some(3),
        greedy: false,
        sub: Box::new(subseq),
    };
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_min_0_max_some_equal_4_greedy_true() {
    let extractor = Extractor::new()
        .limit_repeat(0);
    let subseq = Seq::singleton(Literal::inexact(vec![b'c']));
    let rep = hir::Repetition {
        min: 0,
        max: Some(4),
        greedy: true,
        sub: Box::new(subseq),
    };
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_min_0_max_some_equal_5_greedy_false() {
    let extractor = Extractor::new()
        .limit_repeat(0);
    let subseq = Seq::singleton(Literal::exact(vec![b'd']));
    let rep = hir::Repetition {
        min: 0,
        max: Some(5),
        greedy: false,
        sub: Box::new(subseq),
    };
    extractor.extract_repetition(&rep);
}

