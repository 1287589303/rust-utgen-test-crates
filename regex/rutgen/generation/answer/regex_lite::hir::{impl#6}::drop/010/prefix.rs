// Answer 0

#[test]
fn test_drop_with_empty_alternation() {
    let empty_alternation = HirKind::Alternation(vec![]);
    let hir = Hir {
        kind: empty_alternation,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    let _ = hir; // Calls drop on hir
}

#[test]
fn test_drop_with_empty_concat() {
    let empty_concat = HirKind::Concat(vec![]);
    let sub_hir = Hir {
        kind: empty_concat,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    
    let alternation_hir = HirKind::Alternation(vec![sub_hir]);
    let hir = Hir {
        kind: alternation_hir,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    let _ = hir; // Calls drop on hir
}

#[test]
fn test_drop_with_empty_capture() {
    let empty_capture = Capture {
        index: 0,
        name: None,
        sub: Box::new(Hir::empty()),
    };
    
    let capture_hir = HirKind::Capture(empty_capture);
    let alternation_hir = HirKind::Alternation(vec![Hir {
        kind: capture_hir,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    }]);
    
    let hir = Hir {
        kind: alternation_hir,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    let _ = hir; // Calls drop on hir
}

#[test]
fn test_drop_with_empty_repetition() {
    let empty_repetition = Repetition {
        min: 0,
        max: None,
        greedy: true,
        sub: Box::new(Hir::empty()),
    };
    
    let repetition_hir = HirKind::Repetition(empty_repetition);
    let alternation_hir = HirKind::Alternation(vec![Hir {
        kind: repetition_hir,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    }]);
    
    let hir = Hir {
        kind: alternation_hir,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    let _ = hir; // Calls drop on hir
}

