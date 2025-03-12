// Answer 0

#[test]
fn test_literal_to_tokens() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    let literal = Literal::new(b"test_literal".as_ref(), proc_macro2::Span::call_site());
    let mut tokens = TokenStream::new();

    literal.to_tokens(&mut tokens);
    
    let expected: TokenStream = Literal::new(b"test_literal".as_ref(), proc_macro2::Span::call_site()).into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_literal_to_token_stream() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    let literal = Literal::new(b"test_literal_stream".as_ref(), proc_macro2::Span::call_site());
    let token_stream = literal.to_token_stream();
    
    let expected: TokenStream = Literal::new(b"test_literal_stream".as_ref(), proc_macro2::Span::call_site()).into();
    assert_eq!(token_stream.to_string(), expected.to_string());
}

#[test]
fn test_literal_into_token_stream() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    let literal = Literal::new(b"test_literal_into_stream".as_ref(), proc_macro2::Span::call_site());
    let token_stream = literal.into_token_stream();
    
    let expected: TokenStream = Literal::new(b"test_literal_into_stream".as_ref(), proc_macro2::Span::call_site()).into();
    assert_eq!(token_stream.to_string(), expected.to_string());
}

