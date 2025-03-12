// Answer 0

#[test]
fn test_span_option_none() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let test_fragment = TestIdentFragment;
    let span = test_fragment.span();
    assert_eq!(span, None);
}

#[test]
fn test_span_as_mut_ref() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let mut test_fragment = TestIdentFragment;
    let span = (&mut test_fragment).span();
    assert_eq!(span, None);
}

