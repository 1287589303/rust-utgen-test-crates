// Answer 0

#[test]
fn test_check_hir_nesting_empty() {
    let hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    let _ = check_hir_nesting(&hir, 1);
}

#[test]
fn test_check_hir_nesting_char() {
    let hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = check_hir_nesting(&hir, 1);
}

#[test]
fn test_check_hir_nesting_class() {
    let hir = Hir {
        kind: HirKind::Class(vec!['a', 'b']),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = check_hir_nesting(&hir, 1);
}

#[test]
fn test_check_hir_nesting_look() {
    let hir = Hir {
        kind: HirKind::Look(vec![Hir::new(HirKind::Char('a'))]), // Assuming a dummy method to create Hir
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = check_hir_nesting(&hir, 1);
}

#[test]
fn test_check_hir_nesting_repetition() {
    let sub_hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let hir = Hir {
        kind: HirKind::Repetition(hir::Repetition { sub: Box::new(sub_hir), min: 1, max: None }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = check_hir_nesting(&hir, 1);
}

#[test]
fn test_check_hir_nesting_capture() {
    let sub_hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let hir = Hir {
        kind: HirKind::Capture(hir::Capture { name: None, sub: Box::new(sub_hir) }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = check_hir_nesting(&hir, 1);
}

#[test]
fn test_check_hir_nesting_concat() {
    let sub_hir1 = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let sub_hir2 = Hir {
        kind: HirKind::Char('b'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let hir = Hir {
        kind: HirKind::Concat(vec![Box::new(sub_hir1), Box::new(sub_hir2)]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = check_hir_nesting(&hir, 1);
}

#[test]
fn test_check_hir_nesting_alternation() {
    let sub_hir1 = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let sub_hir2 = Hir {
        kind: HirKind::Char('b'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let hir = Hir {
        kind: HirKind::Alternation(vec![Box::new(sub_hir1), Box::new(sub_hir2)]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = check_hir_nesting(&hir, 1);
}

#[test]
fn test_check_hir_nesting_exceeding_limit() {
    let sub_hir = Hir {
        kind: HirKind::Repetition(hir::Repetition { sub: Box::new(HirKind::Empty.into()), min: 1, max: None }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let hir = Hir {
        kind: HirKind::Capture(hir::Capture { name: None, sub: Box::new(sub_hir) }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = check_hir_nesting(&hir, 0);
}

