// Answer 0

#[test]
fn test_to_tokens() {
    use quote::quote;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestStruct(i32);

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(self.0));
        }
    }

    let value = TestStruct(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected_tokens = quote! { 42i32 };
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

