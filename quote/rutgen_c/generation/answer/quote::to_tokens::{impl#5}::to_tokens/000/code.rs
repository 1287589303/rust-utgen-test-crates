// Answer 0

#[test]
fn test_to_tokens_some_value() {
    struct TestValue;

    impl ToTokens for TestValue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Ident::new("test_value", Span::call_site()));
        }
    }

    let value: Option<&TestValue> = Some(&TestValue);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected_tokens: TokenStream = Ident::new("test_value", Span::call_site()).into();

    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_none_value() {
    let value: Option<&bool> = None;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    assert!(tokens.is_empty());
}

#[test]
fn test_to_tokens_with_bool() {
    let value: Option<&bool> = Some(&true);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    let expected_tokens: TokenStream = Ident::new("true", Span::call_site()).into();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

