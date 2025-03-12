// Answer 0

#[test]
fn test_append_with_token() {
    use proc_macro2::{Ident, TokenStream};

    let mut tokens = TokenStream::new();
    let token = Ident::new("example", proc_macro2::Span::call_site());
    
    tokens.append(token);

    let expected: TokenStream = quote::quote! { example }; 
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_append_with_different_token() {
    use proc_macro2::{Literal, TokenStream};

    let mut tokens = TokenStream::new();
    let token = Literal::new("42", proc_macro2::Span::call_site());

    tokens.append(token);

    let expected: TokenStream = quote::quote! { 42 }; 
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[should_panic]
fn test_append_panics_on_invalid_token() {
    use proc_macro2::{TokenStream};

    let mut tokens = TokenStream::new();
    let invalid_token = 123; // This should not compile, as 123 is not Into<TokenTree>
    
    tokens.append(invalid_token); // This line is expected to compile_fail
}

