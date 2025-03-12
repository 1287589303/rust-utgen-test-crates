// Answer 0

#[test]
fn test_append_separated_with_multiple_tokens() {
    struct TokenA;
    struct TokenB;

    impl ToTokens for TokenA {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream { TokenStream::new() }
        fn into_token_stream(self) -> TokenStream { TokenStream::new() }
    }

    impl ToTokens for TokenB {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream { TokenStream::new() }
        fn into_token_stream(self) -> TokenStream { TokenStream::new() }
    }

    let mut stream = TokenStream::new();
    let tokens = vec![TokenA, TokenB];
    let separator = TokenB;

    stream.append_separated(tokens, separator);
}

#[test]
fn test_append_separated_with_single_item() {
    struct TokenC;

    impl ToTokens for TokenC {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream { TokenStream::new() }
        fn into_token_stream(self) -> TokenStream { TokenStream::new() }
    }

    let mut stream = TokenStream::new();
    let tokens = vec![TokenC];
    let separator = TokenC;

    stream.append_separated(tokens, separator);
}

#[test]
#[should_panic]
fn test_append_separated_with_empty_iter() {
    struct TokenD;

    impl ToTokens for TokenD {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream { TokenStream::new() }
        fn into_token_stream(self) -> TokenStream { TokenStream::new() }
    }

    let mut stream = TokenStream::new();
    let tokens: Vec<TokenD> = vec![];
    let separator = TokenD;

    stream.append_separated(tokens, separator);
}

