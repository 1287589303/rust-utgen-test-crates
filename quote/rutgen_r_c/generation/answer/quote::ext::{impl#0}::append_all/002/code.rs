// Answer 0

#[test]
fn test_append_all_empty_iter() {
    use proc_macro2::TokenStream;

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

    let mut token_stream = TokenStream::new();
    let empty_iter: Vec<DummyToken> = Vec::new();
    token_stream.append_all(empty_iter);
}

#[test]
fn test_append_all_single_token() {
    use proc_macro2::TokenStream;

    struct SingleToken;

    impl ToTokens for SingleToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(self.to_token_stream());
        }
        fn to_token_stream(&self) -> TokenStream {
            TokenStream::from(proc_macro2::TokenTree::Ident(proc_macro2::Ident::new("a", proc_macro2::Span::call_site())))
        }
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    let mut token_stream = TokenStream::new();
    let single_token_iter = vec![SingleToken];
    token_stream.append_all(single_token_iter);
}

