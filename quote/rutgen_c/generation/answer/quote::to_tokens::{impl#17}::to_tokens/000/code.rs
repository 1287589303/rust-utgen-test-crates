// Answer 0

#[test]
fn test_to_tokens_bool_true() {
    use proc_macro2::TokenStream;
    
    struct BoolTrue;

    impl ToTokens for BoolTrue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let value: bool = true;
            tokens.append(Ident::new(if value { "true" } else { "false" }, Span::call_site()));
        }
    }

    let mut tokens = TokenStream::new();
    BoolTrue.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "true");
}

#[test]
fn test_to_tokens_bool_false() {
    use proc_macro2::TokenStream;
    
    struct BoolFalse;

    impl ToTokens for BoolFalse {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let value: bool = false;
            tokens.append(Ident::new(if value { "true" } else { "false" }, Span::call_site()));
        }
    }

    let mut tokens = TokenStream::new();
    BoolFalse.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "false");
}

#[test]
fn test_to_tokens_u64() {
    use proc_macro2::TokenStream;
    
    struct U64Value {
        value: u64,
    }

    impl ToTokens for U64Value {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u64_suffixed(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    U64Value { value: 42 }.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "42u64");
}

