// Answer 0

#[test]
fn test_drop_repetition_with_empty_sub() {
    let empty_hir = Hir::empty();
    let repetition = Repetition {
        min: 0,
        max: None,
        greedy: true,
        sub: Box::new(empty_hir),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = drop(&mut hir);
}

#[test]
fn test_drop_repetition_with_empty_sub2() {
    let empty_hir = Hir::empty();
    let repetition = Repetition {
        min: 1,
        max: None,
        greedy: true,
        sub: Box::new(empty_hir),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = drop(&mut hir);
}

#[test]
fn test_drop_repetition_with_empty_sub3() {
    let empty_hir = Hir::empty();
    let repetition = Repetition {
        min: 0,
        max: Some(5),
        greedy: false,
        sub: Box::new(empty_hir),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = drop(&mut hir);
}

