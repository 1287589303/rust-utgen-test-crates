// Answer 0

#[test]
fn test_to_tokens_with_true() {
    struct BoolWrapper(bool);
    
    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            (*self).to_tokens(tokens);
        }
    }
    
    let value = BoolWrapper(true);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected_tokens: TokenStream = TokenStream::from(Ident::new("true", Span::call_site()));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_with_false() {
    struct BoolWrapper(bool);
    
    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            (*self).to_tokens(tokens);
        }
    }
    
    let value = BoolWrapper(false);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected_tokens: TokenStream = TokenStream::from(Ident::new("false", Span::call_site()));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

