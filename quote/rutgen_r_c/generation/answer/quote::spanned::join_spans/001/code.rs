// Answer 0

#[test]
fn test_join_spans_single_token() {
    use proc_macro2::TokenStream;
    use proc_macro2::TokenTree;
    use proc_macro2::Span;

    let tokens: TokenStream = TokenStream::from(vec![TokenTree::Ident(proc_macro2::Ident::new("test", Span::from_usize(1).unwrap()))]);
    let result = join_spans(tokens);
    assert_eq!(result, Span::from_usize(1).unwrap());
}

#[test]
fn test_join_spans_multiple_tokens() {
    use proc_macro2::TokenStream;
    use proc_macro2::TokenTree;
    use proc_macro2::Span;

    let tokens: TokenStream = TokenStream::from(vec![
        TokenTree::Ident(proc_macro2::Ident::new("test1", Span::from_usize(1).unwrap())),
        TokenTree::Ident(proc_macro2::Ident::new("test2", Span::from_usize(2).unwrap())),
    ]);
    let result = join_spans(tokens);
    assert_eq!(result, Span::join(Span::from_usize(1).unwrap(), Span::from_usize(2).unwrap()).unwrap());
}

#[test]
fn test_join_spans_no_tokens() {
    use proc_macro2::TokenStream;

    let tokens: TokenStream = TokenStream::new();
    let result = join_spans(tokens);
    assert_eq!(result, Span::call_site());
}

