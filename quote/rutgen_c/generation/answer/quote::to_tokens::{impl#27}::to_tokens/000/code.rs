// Answer 0

#[test]
fn test_ident_to_tokens() {
    use proc_macro2::{Ident, TokenStream};
    
    let ident = Ident::new("test_identifier", Span::call_site());
    let mut tokens = TokenStream::new();
    
    ident.to_tokens(&mut tokens);
    
    let expected_tokens: TokenStream = Ident::new("test_identifier", Span::call_site()).into_token_stream();
    
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_ident_into_token_stream() {
    use proc_macro2::{Ident, TokenStream};
    
    let ident = Ident::new("another_identifier", Span::call_site());
    
    let tokens: TokenStream = ident.into_token_stream();
    
    let expected_tokens: TokenStream = Ident::new("another_identifier", Span::call_site()).into_token_stream();
    
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

