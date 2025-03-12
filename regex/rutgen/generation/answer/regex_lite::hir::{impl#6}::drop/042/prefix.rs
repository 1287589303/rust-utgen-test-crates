// Answer 0

#[test]
fn test_drop_repetition_non_empty_sub_alternation() {
    let sub_hir = Hir {
        kind: HirKind::Class(Class::new()), // Assuming Class::new() exists
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let repetition = Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    
    let alternation_hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir {
                kind: HirKind::Char('a'),
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            },
            Hir {
                kind: HirKind::Char('b'),
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            },
        ]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let mut hir_clone = hir.clone();
    std::mem::drop(hir_clone);
}

#[test]
fn test_drop_repetition_non_empty_sub_empty_alternation() {
    let sub_hir = Hir {
        kind: HirKind::Class(Class::new()), // Assuming Class::new() exists
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let repetition = Repetition {
        min: 1,
        max: Some(2),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    
    let alternation_hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir {
                kind: HirKind::Char('c'),
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            },
        ]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let mut hir_clone = hir.clone();
    std::mem::drop(hir_clone);
}

