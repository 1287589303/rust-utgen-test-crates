// Answer 0

#[test]
fn test_cow_to_tokens() {
    struct TestType;

    impl ToTokens for TestType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(iter::once(TokenTree::Ident(Ident::new("test", Span::call_site()))));
        }
    }

    let value = TestType;
    let cow_value: Cow<TestType> = Cow::Borrowed(&value);
    let mut tokens = TokenStream::new();
    cow_value.to_tokens(&mut tokens);

    let expected_tokens: TokenStream = iter::once(TokenTree::Ident(Ident::new("test", Span::call_site()))).collect();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_cow_to_token_stream() {
    struct AnotherTestType;

    impl ToTokens for AnotherTestType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(iter::once(TokenTree::Ident(Ident::new("another_test", Span::call_site()))));
        }
    }

    let value = AnotherTestType;
    let cow_value: Cow<AnotherTestType> = Cow::Borrowed(&value);
    let tokens = cow_value.to_token_stream();

    let expected_tokens: TokenStream = iter::once(TokenTree::Ident(Ident::new("another_test", Span::call_site()))).collect();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

