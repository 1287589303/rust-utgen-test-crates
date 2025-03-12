// Answer 0

#[test]
fn test_extend_non_empty_iterator() {
    let mut token_stream = TokenStream::Compiler(DeferredTokenStream::new());
    let streams = vec![
        TokenStream::Compiler(DeferredTokenStream::new()),
        TokenStream::Fallback(fallback::TokenStream::new()),
    ];
    token_stream.extend(streams.into_iter());
}

#[test]
fn test_extend_empty_iterator() {
    let mut token_stream = TokenStream::Fallback(fallback::TokenStream::new());
    let streams: Vec<TokenStream> = vec![];
    token_stream.extend(streams.into_iter());
}

#[test]
fn test_extend_single_item_iterator() {
    let mut token_stream = TokenStream::Compiler(DeferredTokenStream::new());
    let streams = vec![TokenStream::Fallback(fallback::TokenStream::new())];
    token_stream.extend(streams.into_iter());
}

#[test]
fn test_extend_mixed_variant_iterator() {
    let mut token_stream = TokenStream::Fallback(fallback::TokenStream::new());
    let streams = vec![
        TokenStream::Compiler(DeferredTokenStream::new()),
        TokenStream::Fallback(fallback::TokenStream::new()),
        TokenStream::Compiler(DeferredTokenStream::new()),
    ];
    token_stream.extend(streams.into_iter());
}

#[test]
fn test_extend_large_iterator() {
    let mut token_stream = TokenStream::Compiler(DeferredTokenStream::new());
    let streams: Vec<TokenStream> = (0..1000)
        .map(|_| TokenStream::Fallback(fallback::TokenStream::new()))
        .collect();
    token_stream.extend(streams.into_iter());
}

#[test]
fn test_extend_zero_items() {
    let mut token_stream = TokenStream::Fallback(fallback::TokenStream::new());
    let streams: Vec<TokenStream> = vec![];
    token_stream.extend(streams.into_iter());
}

