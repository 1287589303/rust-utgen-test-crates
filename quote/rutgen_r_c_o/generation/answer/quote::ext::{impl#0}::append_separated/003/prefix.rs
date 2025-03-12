// Answer 0

#[test]
fn test_append_separated_empty_iterator() {
    struct DummyToken;

    impl ToTokens for DummyToken {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream {
            TokenStream::new()
        }
        fn into_token_stream(self) -> TokenStream {
            TokenStream::new()
        }
    }

    let mut tokens = TokenStream::new();
    let separator = DummyToken;
    let empty_iter = iter::empty::<DummyToken>();

    tokens.append_separated(empty_iter, separator);
}

#[test]
fn test_append_separated_empty_iterator_with_different_separator() {
    struct AnotherDummyToken;

    impl ToTokens for AnotherDummyToken {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream {
            TokenStream::new()
        }
        fn into_token_stream(self) -> TokenStream {
            TokenStream::new()
        }
    }

    let mut tokens = TokenStream::new();
    let separator = AnotherDummyToken;
    let empty_iter = iter::empty::<AnotherDummyToken>();

    tokens.append_separated(empty_iter, separator);
}

