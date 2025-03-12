// Answer 0

#[test]
fn test_drop_with_non_empty_repetition() {
    let sub_hir_char = Hir::char('a');
    let sub_repetition = Repetition {
        min: 1,
        max: None,
        greedy: true,
        sub: Box::new(sub_hir_char),
    };
    let repetition_hir = Hir {
        kind: HirKind::Repetition(sub_repetition),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = repetition_hir; // Trigger drop
}

#[test]
fn test_drop_with_repetition_and_class() {
    let sub_hir_class = Hir::class(Class::new()); // Assuming Class::new() is valid for test
    let sub_repetition = Repetition {
        min: 1,
        max: None,
        greedy: true,
        sub: Box::new(sub_hir_class),
    };
    let repetition_hir = Hir {
        kind: HirKind::Repetition(sub_repetition),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = repetition_hir; // Trigger drop
}

#[test]
fn test_drop_with_nested_repetition() {
    let inner_sub_hir_char = Hir::char('b');
    let inner_sub_repetition = Repetition {
        min: 1,
        max: None,
        greedy: true,
        sub: Box::new(inner_sub_hir_char),
    };
    let outer_sub_repetition = Repetition {
        min: 1,
        max: None,
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Repetition(inner_sub_repetition),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };
    let repetition_hir = Hir {
        kind: HirKind::Repetition(outer_sub_repetition),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = repetition_hir; // Trigger drop
}

