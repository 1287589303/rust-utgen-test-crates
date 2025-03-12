// Answer 0

#[test]
fn test_span_with_some() {
    struct TestIdentFragment;
    
    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        
        fn span(&self) -> Option<Span> {
            Some(Span::call_site())
        }
    }
    
    let fragment = &mut TestIdentFragment;
    assert!(fragment.span().is_some());
}

#[test]
fn test_span_with_none() {
    struct TestIdentFragment;
    
    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        
        fn span(&self) -> Option<Span> {
            None
        }
    }
    
    let fragment = &mut TestIdentFragment;
    assert!(fragment.span().is_none());
}

#[test]
fn test_span_with_reference() {
    struct TestIdentFragment;
    
    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn span(&self) -> Option<Span> {
            Some(Span::call_site())
        }
    }
    
    let fragment = TestIdentFragment;
    let fragment_ref = &mut fragment;
    assert!(fragment_ref.span().is_some());
}

