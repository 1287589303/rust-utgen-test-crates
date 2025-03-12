// Answer 0

#[test]
fn test_extract_repetition_min_zero_max_one_greedy_true() {
    let extractor = Extractor::new();
    let rep = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: true,
        sub: Box::new(hir::Hir { kind: hir::HirKind::Literal(hir::Literal(vec![b'a'])), props: Default::default() }),
    };
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_min_zero_max_two_greedy_false() {
    let extractor = Extractor::new();
    let rep = hir::Repetition {
        min: 0,
        max: Some(2),
        greedy: false,
        sub: Box::new(hir::Hir { kind: hir::HirKind::Literal(hir::Literal(vec![b'a'])), props: Default::default() }),
    };
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_min_zero_max_none_greedy_true() {
    let extractor = Extractor::new();
    let rep = hir::Repetition {
        min: 0,
        max: None,
        greedy: true,
        sub: Box::new(hir::Hir { kind: hir::HirKind::Literal(hir::Literal(vec![b'a'])), props: Default::default() }),
    };
    extractor.extract_repetition(&rep);
}

#[test]
fn test_extract_repetition_min_zero_max_none_greedy_false() {
    let extractor = Extractor::new();
    let rep = hir::Repetition {
        min: 0,
        max: None,
        greedy: false,
        sub: Box::new(hir::Hir { kind: hir::HirKind::Literal(hir::Literal(vec![b'a'])), props: Default::default() }),
    };
    extractor.extract_repetition(&rep);
}

