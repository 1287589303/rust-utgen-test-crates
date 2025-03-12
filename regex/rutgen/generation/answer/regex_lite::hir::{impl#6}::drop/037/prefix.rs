// Answer 0

#[test]
fn test_drop_capture_with_non_empty_sub_expression() {
    let capture_sub_hir = Hir::class(Class::new()); // Initializing a non-empty sub expression.
    let capture = Capture {
        index: 0,
        name: None,
        sub: Box::new(capture_sub_hir),
    };
    
    let hir = Hir {
        kind: HirKind::Capture(capture),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };

    // Calling the drop function through the scope of `hir`.
    drop(hir);
}

#[test]
fn test_drop_repetition_with_non_empty_sub_expression() {
    let repetition_sub_hir = Hir::class(Class::new()); // Initializing a non-empty sub expression.
    let repetition = Repetition {
        min: 1,
        max: Some(5),
        greedy: true,
        sub: Box::new(repetition_sub_hir),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };

    // Calling the drop function through the scope of `hir`.
    drop(hir);
}

#[test]
fn test_drop_concat() {
    let first_hir = Hir::class(Class::new()); // Initializing a non-empty expression.
    let second_hir = Hir::class(Class::new()); // Another non-empty expression.
    let concat = HirKind::Concat(vec![first_hir, second_hir]);

    let hir = Hir {
        kind: concat,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };

    // Calling the drop function through the scope of `hir`.
    drop(hir);
}

#[test]
fn test_drop_alternation() {
    let first_hir = Hir::class(Class::new()); // Initializing a non-empty expression.
    let second_hir = Hir::class(Class::new()); // Another non-empty expression.
    let alternation = HirKind::Alternation(vec![first_hir, second_hir]);

    let hir = Hir {
        kind: alternation,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };

    // Calling the drop function through the scope of `hir`.
    drop(hir);
}

