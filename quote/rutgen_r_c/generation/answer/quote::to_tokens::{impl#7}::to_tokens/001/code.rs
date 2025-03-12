// Answer 0

#[test]
fn test_to_tokens_true() {
    struct TestBool(bool);

    impl ToTokens for TestBool {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let word = if self.0 { "true" } else { "false" };
            tokens.append(Ident::new(word, Span::call_site()));
        }
    }

    let value = TestBool(true);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected: TokenStream = quote::quote! { true }.into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_false() {
    struct TestBool(bool);

    impl ToTokens for TestBool {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let word = if self.0 { "true" } else { "false" };
            tokens.append(Ident::new(word, Span::call_site()));
        }
    }

    let value = TestBool(false);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected: TokenStream = quote::quote! { false }.into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

