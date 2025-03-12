// Answer 0

#[test]
fn test_to_tokens_clones_token_tree() {
    use proc_macro2::{TokenTree, Ident, TokenStream};

    let ident = Ident::new("my_ident", Span::call_site());
    let token_tree = TokenTree::Ident(ident);
    let mut tokens = TokenStream::new();

    token_tree.to_tokens(&mut tokens);
    
    let expected_tokens: TokenStream = TokenTree::Ident(Ident::new("my_ident", Span::call_site())).into();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_with_multiple_token_trees() {
    use proc_macro2::{TokenTree, Literal, TokenStream};

    let lit = TokenTree::Literal(Literal::new("42", Span::call_site()));
    let ident = TokenTree::Ident(Ident::new("my_var", Span::call_site()));

    let mut tokens = TokenStream::new();
    lit.to_tokens(&mut tokens);
    ident.to_tokens(&mut tokens);

    let expected_tokens: TokenStream = 
        TokenTree::Literal(Literal::new("42", Span::call_site())).into() +
        TokenTree::Ident(Ident::new("my_var", Span::call_site())).into();
    
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_empty_token_tree() {
    use proc_macro2::{TokenTree, TokenStream};

    let token_tree: TokenTree = TokenTree::Group(Group::new(Span::call_site(), TokenStream::new()));
    let mut tokens = TokenStream::new();

    token_tree.to_tokens(&mut tokens);

    assert!(tokens.is_empty());
}

