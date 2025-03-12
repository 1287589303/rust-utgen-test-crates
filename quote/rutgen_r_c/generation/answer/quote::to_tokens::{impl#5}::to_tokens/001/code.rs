// Answer 0

#[test]
fn test_to_tokens_bool_true() {
    use proc_macro2::TokenStream;
    let value: Option<&bool> = Some(&true);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected: TokenStream = TokenStream::from(Ident::new("true", Span::call_site()));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_bool_false() {
    use proc_macro2::TokenStream;
    let value: Option<&bool> = Some(&false);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected: TokenStream = TokenStream::from(Ident::new("false", Span::call_site()));
    assert_eq!(tokens.to_string(), expected.to_string());
}

