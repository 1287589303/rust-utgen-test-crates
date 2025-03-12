// Answer 0

#[test]
fn test_repetition_with_non_empty_sub() {
    let sub_hir = Hir {
        kind: HirKind::Char('a'),
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
    
    let hir_kind = HirKind::Repetition(repetition);
    
    let subs: &[Hir] = hir_kind.subs();
}

#[test]
fn test_repetition_with_empty_sub() {
    let sub_hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    
    let repetition = Repetition {
        min: 0,
        max: Some(5),
        greedy: false,
        sub: Box::new(sub_hir),
    };
    
    let hir_kind = HirKind::Repetition(repetition);
    
    let subs: &[Hir] = hir_kind.subs();
}

#[test]
fn test_repetition_with_single_min() {
    let sub_hir = Hir {
        kind: HirKind::Class(Class { ranges: vec![] }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let repetition = Repetition {
        min: 1,
        max: Some(1),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    
    let hir_kind = HirKind::Repetition(repetition);
    
    let subs: &[Hir] = hir_kind.subs();
}

#[test]
fn test_repetition_with_large_range() {
    let sub_hir = Hir {
        kind: HirKind::Capture(Capture {
            index: 0,
            name: None,
            sub: Box::new(Hir {
                kind: HirKind::Char('b'),
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            }),
        }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let repetition = Repetition {
        min: 0,
        max: None,
        greedy: false,
        sub: Box::new(sub_hir),
    };
    
    let hir_kind = HirKind::Repetition(repetition);
    
    let subs: &[Hir] = hir_kind.subs();
}

#[test]
fn test_repetition_with_zero_min() {
    let sub_hir = Hir {
        kind: HirKind::Char('c'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let repetition = Repetition {
        min: 0,
        max: Some(2),
        greedy: false,
        sub: Box::new(sub_hir),
    };

    let hir_kind = HirKind::Repetition(repetition);
    
    let subs: &[Hir] = hir_kind.subs();
}

