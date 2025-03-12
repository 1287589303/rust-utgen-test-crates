// Answer 0

#[test]
fn test_into_token_stream_basic() {
    struct TestStruct;

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let ident = Ident::new("test", Span::call_site());
            tokens.extend(iter::once(TokenTree::Ident(ident)));
        }
    }

    let test_instance = TestStruct;
    let token_stream = test_instance.into_token_stream();
    let expected: TokenStream = quote::quote! { test }.into();
    assert_eq!(token_stream.to_string(), expected.to_string());
}

#[test]
fn test_into_token_stream_empty() {
    struct EmptyStruct;

    impl ToTokens for EmptyStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            // No tokens added
        }
    }

    let empty_instance = EmptyStruct;
    let token_stream = empty_instance.into_token_stream();
    let expected: TokenStream = TokenStream::new();  // An empty TokenStream
    assert_eq!(token_stream.to_string(), expected.to_string());
}

