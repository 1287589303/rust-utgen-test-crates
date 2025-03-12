// Answer 0

#[test]
fn test_to_tokens_with_positive_i16() {
    struct PositiveI16(i16);
    
    impl ToTokens for PositiveI16 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i16_suffixed(self.0));
        }
    }
    
    let value = PositiveI16(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected: TokenStream = TokenStream::from(Literal::i16_suffixed(42));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_negative_i16() {
    struct NegativeI16(i16);
    
    impl ToTokens for NegativeI16 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i16_suffixed(self.0));
        }
    }
    
    let value = NegativeI16(-42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected: TokenStream = TokenStream::from(Literal::i16_suffixed(-42));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_zero_i16() {
    struct ZeroI16(i16);
    
    impl ToTokens for ZeroI16 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i16_suffixed(self.0));
        }
    }
    
    let value = ZeroI16(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected: TokenStream = TokenStream::from(Literal::i16_suffixed(0));
    assert_eq!(tokens.to_string(), expected.to_string());
}

