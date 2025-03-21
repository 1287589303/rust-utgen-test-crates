// Answer 0

#[test]
fn test_alternation_with_multiple_hir() {
    struct TestHir {
        is_start_anchored: bool,
        is_match_empty: bool,
        static_explicit_captures_len: Option<usize>,
    }

    impl TestHir {
        fn new(is_start_anchored: bool, is_match_empty: bool, static_explicit_captures_len: Option<usize>) -> Hir {
            Hir {
                kind: HirKind::Empty,
                is_start_anchored,
                is_match_empty,
                static_explicit_captures_len,
            }
        }
    }

    let subs = vec![
        TestHir::new(true, false, Some(1)),   // First Hir with is_start_anchored = true
        TestHir::new(false, true, Some(2)),   // Second Hir with is_match_empty = true
        TestHir::new(false, false, Some(3)),  // Third Hir
    ];

    let result = Hir::alternation(subs.into_iter().map(|h| h).collect());

    drop(result); // To prevent unused variable warning
}

#[test]
fn test_alternation_with_start_anchored_and_empty() {
    struct TestHir {
        is_start_anchored: bool,
        is_match_empty: bool,
        static_explicit_captures_len: Option<usize>,
    }

    impl TestHir {
        fn new(is_start_anchored: bool, is_match_empty: bool, static_explicit_captures_len: Option<usize>) -> Hir {
            Hir {
                kind: HirKind::Empty,
                is_start_anchored,
                is_match_empty,
                static_explicit_captures_len,
            }
        }
    }

    let subs = vec![
        TestHir::new(true, false, Some(3)),   // First Hir with is_start_anchored = true
        TestHir::new(false, true, Some(1)),    // Second Hir with is_match_empty = true
        TestHir::new(false, false, Some(2)),   // Third Hir
    ];

    let result = Hir::alternation(subs.into_iter().map(|h| h).collect());

    drop(result); // To prevent unused variable warning
}

