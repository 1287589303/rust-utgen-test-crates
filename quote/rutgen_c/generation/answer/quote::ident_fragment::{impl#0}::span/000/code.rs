// Answer 0

#[test]
fn test_span_with_ident_fragment() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let fragment = TestIdentFragment;
    assert_eq!(fragment.span(), None);
}

#[test]
fn test_span_with_mut_reference() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let mut fragment = TestIdentFragment;
    let fragment_ref = &mut fragment;
    assert_eq!(fragment_ref.span(), None);
}

