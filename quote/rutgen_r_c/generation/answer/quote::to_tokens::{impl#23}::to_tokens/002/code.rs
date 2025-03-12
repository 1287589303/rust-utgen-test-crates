// Answer 0

#[test]
fn test_to_tokens_false() {
    use proc_macro2::TokenStream;
    use proc_macro2::Ident;
    use proc_macro2::Span;

    let value: bool = false;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected = TokenStream::from(Ident::new("false", Span::call_site()));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_true() {
    use proc_macro2::TokenStream;
    use proc_macro2::Ident;
    use proc_macro2::Span;

    let value: bool = true;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected = TokenStream::from(Ident::new("true", Span::call_site()));
    assert_eq!(tokens.to_string(), expected.to_string());
}

