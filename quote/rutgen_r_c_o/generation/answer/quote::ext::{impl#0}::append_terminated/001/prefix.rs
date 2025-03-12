// Answer 0

#[test]
fn test_append_terminated_with_tokens() {
    struct TokenImpl;

    impl ToTokens for TokenImpl {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream {
            TokenStream::new()
        }
        fn into_token_stream(self) -> TokenStream {
            TokenStream::new()
        }
    }

    let mut tokens = TokenStream::new();
    let items = vec![TokenImpl];
    let terminator = TokenImpl;

    tokens.append_terminated(items, terminator);
}

#[test]
fn test_append_terminated_with_empty_iter() {
    struct TokenImpl;

    impl ToTokens for TokenImpl {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream {
            TokenStream::new()
        }
        fn into_token_stream(self) -> TokenStream {
            TokenStream::new()
        }
    }

    let mut tokens = TokenStream::new();
    let items: Vec<TokenImpl> = Vec::new();
    let terminator = TokenImpl;

    tokens.append_terminated(items, terminator);
}

