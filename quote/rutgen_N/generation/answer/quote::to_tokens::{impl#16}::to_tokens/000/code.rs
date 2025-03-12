// Answer 0

#[test]
fn test_to_tokens() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct(u32);

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u32_suffixed(self.0));
        }
    }

    let test_value = TestStruct(42);
    let mut tokens = TokenStream::new();
    test_value.to_tokens(&mut tokens);

    assert_eq!(tokens.to_string(), "42_u32");
}

