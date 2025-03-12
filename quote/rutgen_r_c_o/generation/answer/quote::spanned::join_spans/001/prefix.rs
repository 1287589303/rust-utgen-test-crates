// Answer 0

#[test]
fn test_join_spans_with_multiple_tokens() {
    use proc_macro2::{Ident, TokenStream};

    let tokens = TokenStream::from(Ident::new("token1", Span::call_site()))
        .into_iter()
        .chain(TokenStream::from(Ident::new("token2", Span::call_site())).into_iter())
        .collect::<TokenStream>();

    let _result = join_spans(tokens);
}

#[test]
fn test_join_spans_with_different_spans() {
    use proc_macro2::{Ident, TokenStream};

    let tokens = TokenStream::from(Ident::new("token1", Span::call_site()))
        .into_iter()
        .chain(TokenStream::from(Ident::new("token2", Span::from(Span::call_site().start() + 1))).into_iter())
        .collect::<TokenStream>();

    let _result = join_spans(tokens);
}

#[test]
fn test_join_spans_with_three_tokens() {
    use proc_macro2::{Ident, TokenStream};

    let tokens = TokenStream::from(Ident::new("token1", Span::call_site()))
        .into_iter()
        .chain(TokenStream::from(Ident::new("token2", Span::call_site())).into_iter())
        .chain(TokenStream::from(Ident::new("token3", Span::from(Span::call_site().start() + 2))).into_iter())
        .collect::<TokenStream>();

    let _result = join_spans(tokens);
}

