// Answer 0

#[derive(Debug)]
struct TestIdentFragment;

impl IdentFragment for TestIdentFragment {
    fn span(&self) -> Option<Span> {
        Some(Span { start: 0, end: 1 })
    }
}

#[test]
fn test_ident_fragment_span() {
    let fragment = TestIdentFragment;
    let result = fragment.span();
    assert_eq!(result, Some(Span { start: 0, end: 1 }));
}

#[test]
fn test_ident_fragment_span_none() {
    struct NoneIdentFragment;

    impl IdentFragment for NoneIdentFragment {
        fn span(&self) -> Option<Span> {
            None
        }
    }

    let fragment = NoneIdentFragment;
    let result = fragment.span();
    assert_eq!(result, None);
}

#[test]
fn test_ident_fragment_span_boundary() {
    struct BoundaryIdentFragment;

    impl IdentFragment for BoundaryIdentFragment {
        fn span(&self) -> Option<Span> {
            Some(Span { start: usize::MAX, end: usize::MAX })
        }
    }

    let fragment = BoundaryIdentFragment;
    let result = fragment.span();
    assert_eq!(result, Some(Span { start: usize::MAX, end: usize::MAX }));
}

