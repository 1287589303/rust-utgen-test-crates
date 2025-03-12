// Answer 0

#[test]
fn test_span_for_ident_fragment() {
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
fn test_span_for_another_ident_fragment() {
    struct AnotherTestIdentFragment;

    impl IdentFragment for AnotherTestIdentFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let another_fragment = AnotherTestIdentFragment;
    assert_eq!(another_fragment.span(), None);
}

