// Answer 0

#[test]
fn test_repetition_min_gt_zero_and_max_none() {
    let sub_hir = Hir::empty(); // sub.is_match_empty == true
    let rep = Repetition {
        min: 1,
        max: None,
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

#[test]
fn test_repetition_min_gt_zero_with_non_empty_captures() {
    let sub_hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(2), // greater than 0
    };
    let rep = Repetition {
        min: 1,
        max: None,
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

#[test]
fn test_repetition_min_gt_zero_with_non_empty_captures_non_greedy() {
    let sub_hir = Hir {
        kind: HirKind::Char('b'),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(3), // greater than 0
    };
    let rep = Repetition {
        min: 2,
        max: None,
        greedy: false,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

