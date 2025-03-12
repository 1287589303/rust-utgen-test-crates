// Answer 0

#[test]
fn test_to_tokens_u32() {
    struct U32Wrapper(u32);
    
    impl ToTokens for U32Wrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u32_suffixed(self.0));
        }
    }

    let value = U32Wrapper(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    // Verifying that tokens represent the expected literal
    let expected: TokenStream = "42u32".parse().unwrap();
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_zero() {
    struct U32Wrapper(u32);
    
    impl ToTokens for U32Wrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u32_suffixed(self.0));
        }
    }

    let value = U32Wrapper(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    // Verifying that tokens represent the expected literal
    let expected: TokenStream = "0u32".parse().unwrap();
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_max_u32() {
    struct U32Wrapper(u32);
    
    impl ToTokens for U32Wrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u32_suffixed(self.0));
        }
    }

    let value = U32Wrapper(u32::MAX);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    // Verifying that tokens represent the expected literal
    let expected: TokenStream = "4294967295u32".parse().unwrap();
    assert_eq!(tokens.to_string(), expected.to_string());
}

