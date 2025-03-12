// Answer 0

#[test]
fn test_span_option_none() {
    struct Dummy;
    
    impl IdentFragment for Dummy {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }
    
    let dummy = Dummy;
    assert_eq!(dummy.span(), None);
}

#[test]
fn test_span_option_some() {
    struct DummyWithSpan {
        span: Span,
    }
    
    impl IdentFragment for DummyWithSpan {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn span(&self) -> Option<Span> {
            Some(self.span)
        }
    }
    
    let dummy_with_span = DummyWithSpan { span: Span::call_site() };
    assert_eq!(dummy_with_span.span().is_some(), true);
}

