// Answer 0

#[test]
fn test_span_none() {
    struct IdentFragment;

    impl IdentFragment {
        fn span(&self) -> Option<Span> {
            None
        }
    }

    let ident_fragment = IdentFragment;
    assert_eq!(ident_fragment.span(), None);
}

