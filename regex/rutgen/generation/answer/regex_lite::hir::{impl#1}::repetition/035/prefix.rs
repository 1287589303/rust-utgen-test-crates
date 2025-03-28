// Answer 0

#[test]
fn test_repetition_min_zero_max_some_one() {
    struct DummyHir {
        is_start_anchored: bool,
        is_match_empty: bool,
        static_explicit_captures_len: Option<usize>,
    }

    impl Hir {
        fn new_dummy_hir() -> Hir {
            Hir {
                kind: HirKind::Char('a'),
                is_start_anchored: false,
                is_match_empty: true,
                static_explicit_captures_len: Some(0),
            }
        }

        fn is_start_anchored(&self) -> bool {
            self.is_start_anchored
        }

        fn is_match_empty(&self) -> bool {
            self.is_match_empty
        }

        fn static_explicit_captures_len(&self) -> Option<usize> {
            self.static_explicit_captures_len
        }
    }

    let sub_hir = Box::new(Hir::new_dummy_hir());
    let repetition = Repetition {
        min: 0,
        max: Some(1),
        greedy: true,
        sub: sub_hir,
    };

    let result = Hir::repetition(repetition);
}

