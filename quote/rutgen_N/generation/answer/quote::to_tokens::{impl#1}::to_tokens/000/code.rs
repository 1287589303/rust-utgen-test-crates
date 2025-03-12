// Answer 0

#[test]
fn test_to_tokens() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;

    struct TestStruct;

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { TestStruct });
        }
    }

    let test_instance = TestStruct;
    let mut tokens = TokenStream::new();
    
    test_instance.to_tokens(&mut tokens);
    
    let expected_tokens: TokenStream = quote::quote! { TestStruct }.into();
    
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

