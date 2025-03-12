// Answer 0

#[test]
fn test_to_tokens_with_punct() {
    use proc_macro2::{Punct, Span};

    let mut tokens = TokenStream::new();
    let punct = Punct::new('!', Spans::call(0));
   
    punct.to_tokens(&mut tokens);
   
    let expected_tokens = TokenStream::from(TokenTree::Punct(punct.clone()));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_empty_token_stream() {
    use proc_macro2::Punct;

    let mut tokens = TokenStream::new();
    let punct = Punct::new('.', Span::call_site());
    
    punct.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "." );
}

