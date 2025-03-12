// Answer 0

#[test]
fn test_append_terminated_empty_iterator() {
    let mut tokens = TokenStream::new();
    let term = DummyToken; // assume DummyToken implements ToTokens
    let iter: Vec<()>::into_iter(); // empty iterator of a type that does not implement ToTokens
    tokens.append_terminated(iter, term);
}

#[test]
fn test_append_terminated_one_element_non_to_tokens() {
    struct NotToTokens; // this struct does not implement ToTokens

    let mut tokens = TokenStream::new();
    let term = DummyToken; // assume DummyToken implements ToTokens
    let iter = vec![NotToTokens].into_iter(); // one element that does not implement ToTokens
    tokens.append_terminated(iter, term);
} 

#[test]
fn test_append_terminated_one_element_to_tokens() {
    struct DummyToken; // this struct does implement ToTokens
    impl ToTokens for DummyToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream { TokenStream::new() }
        fn into_token_stream(self) -> TokenStream { TokenStream::new() }
    }

    let mut tokens = TokenStream::new();
    let term = DummyToken; // acceptable U that implements ToTokens
    let iter = vec![DummyToken].into_iter(); // valid iterator with one element 
    tokens.append_terminated(iter, term);
}

