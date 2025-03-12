// Answer 0

#[test]
fn test_to_tokens() {
    use quote::quote;
    use proc_macro2::TokenStream;

    struct TestStruct;

    impl AsRef<TestStruct> for TestStruct {
        fn as_ref(&self) -> &TestStruct {
            self
        }
    }

    impl quote::ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote! { TestStruct });
        }
    }

    let mut tokens = TokenStream::new();
    let test_instance = TestStruct;

    test_instance.to_tokens(&mut tokens);
    
    let expected_tokens: TokenStream = quote! { TestStruct };
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

