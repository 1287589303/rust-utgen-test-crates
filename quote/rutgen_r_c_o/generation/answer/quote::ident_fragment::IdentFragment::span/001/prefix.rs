// Answer 0

#[test]
fn test_ident_fragment_span_none() {
    struct TestFragment;

    impl IdentFragment for TestFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let fragment = TestFragment;
    let result = fragment.span();
}

#[test]
fn test_another_ident_fragment_span_none() {
    struct AnotherTestFragment;

    impl IdentFragment for AnotherTestFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let fragment = AnotherTestFragment;
    let result = fragment.span();
}

