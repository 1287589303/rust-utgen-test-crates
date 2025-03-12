// Answer 0

#[test]
fn test_to_tokens_with_positive_isize() {
    struct DummyIsize(i64);

    impl DummyIsize {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0));
        }
    }

    let value = DummyIsize(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "42isize");
}

#[test]
fn test_to_tokens_with_zero_isize() {
    struct DummyIsize(i64);

    impl DummyIsize {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0));
        }
    }
    
    let value = DummyIsize(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "0isize");
}

#[test]
fn test_to_tokens_with_negative_isize() {
    struct DummyIsize(i64);

    impl DummyIsize {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0));
        }
    }

    let value = DummyIsize(-42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "-42isize");
}

