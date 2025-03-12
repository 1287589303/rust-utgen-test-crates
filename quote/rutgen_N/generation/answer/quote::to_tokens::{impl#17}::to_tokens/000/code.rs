// Answer 0

#[test]
fn test_to_tokens() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct(u64);

    let test_value = TestStruct(42);
    let mut tokens = TokenStream::new();

    test_value.to_tokens(&mut tokens);

    assert_eq!(tokens.to_string(), "42u64");
}

