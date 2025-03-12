// Answer 0

#[test]
fn test_to_tokens_true() {
    use proc_macro2::TokenStream;
    use proc_macro2::Ident;
    use proc_macro2::Span;

    let value: bool = true;
    let mut tokens = TokenStream::new();

    value.to_tokens(&mut tokens);

    let expected_token = Ident::new("true", Span::call_site());
    let expected_stream = TokenStream::from(expected_token);
    
    assert_eq!(tokens.to_string(), expected_stream.to_string());
}

#[test]
fn test_to_tokens_false() {
    use proc_macro2::TokenStream;
    use proc_macro2::Ident;
    use proc_macro2::Span;

    let value: bool = false;
    let mut tokens = TokenStream::new();

    value.to_tokens(&mut tokens);

    let expected_token = Ident::new("false", Span::call_site());
    let expected_stream = TokenStream::from(expected_token);

    assert_eq!(tokens.to_string(), expected_stream.to_string());
}

