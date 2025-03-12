// Answer 0

#[test]
fn test_to_tokens() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct(i64);

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i64_suffixed(self.0));
        }
    }

    let value = TestStruct(42);
    let mut tokens = TokenStream::new();
    
    value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "42i64");
}

