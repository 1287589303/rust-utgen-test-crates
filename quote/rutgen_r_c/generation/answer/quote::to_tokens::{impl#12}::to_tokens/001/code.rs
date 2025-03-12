// Answer 0

#[test]
fn test_to_tokens_positive_i128() {
    struct PositiveI128(i128);
    
    impl ToTokens for PositiveI128 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i128_suffixed(self.0));
        }
    }
    
    let mut tokens = TokenStream::new();
    let value = PositiveI128(123);
    value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "123i128"); // Assuming the format is like this
}

#[test]
fn test_to_tokens_negative_i128() {
    struct NegativeI128(i128);
    
    impl ToTokens for NegativeI128 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i128_suffixed(self.0));
        }
    }
    
    let mut tokens = TokenStream::new();
    let value = NegativeI128(-123);
    value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "-123i128"); // Assuming the format is like this
}

#[test]
fn test_to_tokens_zero_i128() {
    struct ZeroI128(i128);
    
    impl ToTokens for ZeroI128 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i128_suffixed(self.0));
        }
    }
    
    let mut tokens = TokenStream::new();
    let value = ZeroI128(0);
    value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "0i128"); // Assuming the format is like this
}

