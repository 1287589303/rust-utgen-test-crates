// Answer 0

#[test]
fn test_to_tokens_u64() {
    struct TestU64(u64);

    impl ToTokens for TestU64 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u64_suffixed(self.0));
        }
    }

    let value = TestU64(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected: TokenStream = Literal::u64_suffixed(42).into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_zero_u64() {
    struct TestZeroU64(u64);

    impl ToTokens for TestZeroU64 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u64_suffixed(self.0));
        }
    }

    let value = TestZeroU64(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected: TokenStream = Literal::u64_suffixed(0).into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_large_u64() {
    struct TestLargeU64(u64);

    impl ToTokens for TestLargeU64 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u64_suffixed(self.0));
        }
    }

    let value = TestLargeU64(u64::MAX);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected: TokenStream = Literal::u64_suffixed(u64::MAX).into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

