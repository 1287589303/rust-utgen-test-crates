// Answer 0

#[test]
fn test_to_tokens_float() {
    struct FloatToken(f32);
    
    impl ToTokens for FloatToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f32_suffixed(self.0));
        }
    }

    let value = FloatToken(3.14);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::f32_suffixed(3.14));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_negative_float() {
    struct FloatToken(f32);
    
    impl ToTokens for FloatToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f32_suffixed(self.0));
        }
    }

    let value = FloatToken(-2.71);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::f32_suffixed(-2.71));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_zero_float() {
    struct FloatToken(f32);
    
    impl ToTokens for FloatToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f32_suffixed(self.0));
        }
    }

    let value = FloatToken(0.0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::f32_suffixed(0.0));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_large_float() {
    struct FloatToken(f32);
    
    impl ToTokens for FloatToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f32_suffixed(self.0));
        }
    }

    let value = FloatToken(1_000_000.0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::f32_suffixed(1_000_000.0));
    assert_eq!(tokens.to_string(), expected.to_string());
}

