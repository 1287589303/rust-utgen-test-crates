// Answer 0

#[test]
fn test_to_tokens() {
    use quote::quote;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.value));
        }
    }

    let test_instance = TestStruct {
        value: "Hello, World!",
    };
    let mut tokens = TokenStream::new();

    test_instance.to_tokens(&mut tokens);
    let expected = quote! { "Hello, World!" };
    
    assert_eq!(tokens.to_string(), expected.to_string());
}

