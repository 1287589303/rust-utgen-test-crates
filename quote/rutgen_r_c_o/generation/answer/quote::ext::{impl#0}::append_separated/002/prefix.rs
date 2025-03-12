// Answer 0

#[test]
fn test_append_separated_single_item() {
    struct TokenItem;

    impl ToTokens for TokenItem {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream {
            TokenStream::new()
        }
        fn into_token_stream(self) -> TokenStream {
            TokenStream::new()
        }
    }

    let mut tokens = TokenStream::new();
    let single_item = vec![TokenItem];
    tokens.append_separated(single_item.iter(), TokenItem);
}

#[test]
fn test_append_separated_empty_iterator() {
    struct Separator;

    impl ToTokens for Separator {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream {
            TokenStream::new()
        }
        fn into_token_stream(self) -> TokenStream {
            TokenStream::new()
        }
    }

    let mut tokens = TokenStream::new();
    let empty_iter: Vec<TokenItem> = vec![];
    tokens.append_separated(empty_iter.iter(), Separator);
}

