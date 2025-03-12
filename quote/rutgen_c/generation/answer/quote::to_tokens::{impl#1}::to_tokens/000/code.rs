// Answer 0

#[test]
fn test_to_tokens_with_true() {
    use proc_macro2::TokenStream;
    
    struct BoolWrapper(bool);

    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.to_tokens(tokens);
        }
    }

    let mut tokens = TokenStream::new();
    BoolWrapper(true).to_tokens(&mut tokens);
    
    let expected = TokenStream::from(Ident::new("true", Span::call_site()));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_false() {
    use proc_macro2::TokenStream;

    struct BoolWrapper(bool);

    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.to_tokens(tokens);
        }
    }

    let mut tokens = TokenStream::new();
    BoolWrapper(false).to_tokens(&mut tokens);

    let expected = TokenStream::from(Ident::new("false", Span::call_site()));
    assert_eq!(tokens.to_string(), expected.to_string());
}

