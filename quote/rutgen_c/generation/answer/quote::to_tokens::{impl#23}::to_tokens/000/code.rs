// Answer 0

#[test]
fn test_to_tokens_true() {
    use proc_macro2::TokenStream;
    use proc_macro2::Ident;
    use proc_macro2::Span;

    let mut tokens = TokenStream::new();
    let value: bool = true;
    value.to_tokens(&mut tokens);

    let expected_token = Ident::new("true", Span::call_site());
    assert!(tokens.to_string().contains(expected_token.to_string()));
}

#[test]
fn test_to_tokens_false() {
    use proc_macro2::TokenStream;
    use proc_macro2::Ident;
    use proc_macro2::Span;

    let mut tokens = TokenStream::new();
    let value: bool = false;
    value.to_tokens(&mut tokens);

    let expected_token = Ident::new("false", Span::call_site());
    assert!(tokens.to_string().contains(expected_token.to_string()));
}

