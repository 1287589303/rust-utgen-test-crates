// Answer 0

#[test]
fn test_span_with_some_span() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        
        fn span(&self) -> Option<Span> {
            Some(Span::call_site())
        }
    }

    let cow: Cow<TestIdentFragment> = Cow::Owned(TestIdentFragment);
    assert!(cow.span().is_some());
}

#[test]
fn test_span_with_none_span() {
    struct TestIdentFragmentNone;

    impl IdentFragment for TestIdentFragmentNone {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let cow: Cow<TestIdentFragmentNone> = Cow::Owned(TestIdentFragmentNone);
    assert!(cow.span().is_none());
}

