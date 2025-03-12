// Answer 0

#[test]
fn test_join_spans_with_two_tokens() {
    struct MockTokenStream(&'static [&'static str]);
    
    impl Iterator for MockTokenStream {
        type Item = MockToken;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0.is_empty() {
                None
            } else {
                let token = MockToken { span: self.0[0] };
                self.0 = &self.0[1..];
                Some(token)
            }
        }
    }

    struct MockToken {
        span: &'static str,
    }

    impl MockToken {
        fn span(&self) -> Span {
            Span { value: self.span }
        }
    }

    #[derive(Debug)]
    struct Span {
        value: &'static str,
    }

    impl Span {
        fn call_site() -> Span {
            Span { value: "call_site" }
        }

        fn join(self, _other: Span) -> Span {
            self // Simplified join for testing purposes
        }
    }

    let tokens = MockTokenStream(&["span1", "span2"]);
    let result = join_spans(tokens);
    
    assert_eq!(result.value, "span2");
}

#[test]
fn test_join_spans_single_token() {
    struct MockTokenStream(&'static [&'static str]);
    
    impl Iterator for MockTokenStream {
        type Item = MockToken;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0.is_empty() {
                None
            } else {
                let token = MockToken { span: self.0[0] };
                self.0 = &self.0[1..];
                Some(token)
            }
        }
    }

    struct MockToken {
        span: &'static str,
    }

    impl MockToken {
        fn span(&self) -> Span {
            Span { value: self.span }
        }
    }

    #[derive(Debug)]
    struct Span {
        value: &'static str,
    }

    impl Span {
        fn call_site() -> Span {
            Span { value: "call_site" }
        }

        fn join(self, _other: Span) -> Span {
            self // Simplified join for testing purposes
        }
    }

    let tokens = MockTokenStream(&["only_span"]);
    let result = join_spans(tokens);
    
    assert_eq!(result.value, "only_span");
}

