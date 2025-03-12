// Answer 0

#[test]
fn test_hir_drop_repetition_with_non_empty_sub_and_concat() {
    let sub_hir = Hir::char('a'); // non-empty sub-expression
    let repetition = Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(sub_hir),
    };

    let concat_hirs = vec![
        Hir::char('b'),
        Hir::char('c'),
    ];
    let kind = HirKind::Repetition(repetition);
    let hir = Hir {
        kind,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let concat_kind = HirKind::Concat(concat_hirs);
    let mut stack = vec![hir];

    while let Some(mut expr) = stack.pop() {
        if let HirKind::Repetition(ref mut r) = expr.kind {
            if !r.sub.kind.subs().is_empty() {
                stack.push(Hir::concat(vec![])); // add an empty concat to allow further processing
            }
        }
    }
}

#[test]
fn test_hir_drop_repetition_with_large_concat() {
    let sub_hir = Hir::char('x'); // non-empty sub-expression
    let repetition = Repetition {
        min: 1,
        max: Some(2),
        greedy: true,
        sub: Box::new(sub_hir),
    };

    let mut concat_hirs = Vec::new();
    for i in 0..10 {
        concat_hirs.push(Hir::char('y')); // create a larger concat
    }
    let kind = HirKind::Repetition(repetition);
    let hir = Hir {
        kind,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let concat_kind = HirKind::Concat(concat_hirs);
    let mut stack = vec![hir];

    while let Some(mut expr) = stack.pop() {
        if let HirKind::Repetition(ref mut r) = expr.kind {
            if !r.sub.kind.subs().is_empty() {
                stack.push(Hir::concat(vec![])); // add an empty concat to allow further processing
            }
        }
    }
}

