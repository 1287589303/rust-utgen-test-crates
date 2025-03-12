// Answer 0

#[test]
fn drop_with_non_empty_alternation_containing_repetition() {
    let repetition = Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(Hir::char('a')),
    };
    
    let alternation = HirKind::Alternation(vec![
        Hir::repetition(repetition.clone()),
        Hir::repetition(repetition),
    ]);

    let hir_instance = Hir {
        kind: alternation,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = hir_instance;  // Calling drop implicitly through the value.
}

#[test]
fn drop_with_empty_sub_repetition_in_alternation() {
    let empty_repetition = Repetition {
        min: 0,
        max: None,
        greedy: true,
        sub: Box::new(Hir::empty()),  // sub expression is empty
    };

    let alternation = HirKind::Alternation(vec![
        Hir::repetition(empty_repetition),
        Hir::char('b'),  // includes a non-empty expression
    ]);

    let hir_instance = Hir {
        kind: alternation,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = hir_instance;  // Calling drop implicitly through the value.
}

#[test]
fn drop_with_multiple_repetitions_in_alternation() {
    let repetition_a = Repetition {
        min: 2,
        max: Some(5),
        greedy: true,
        sub: Box::new(Hir::char('c')),
    };

    let repetition_b = Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(Hir::char('d')),
    };

    let alternation = HirKind::Alternation(vec![
        Hir::repetition(repetition_a),
        Hir::repetition(repetition_b),
    ]);

    let hir_instance = Hir {
        kind: alternation,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };

    let _ = hir_instance;  // Calling drop implicitly through the value.
}

