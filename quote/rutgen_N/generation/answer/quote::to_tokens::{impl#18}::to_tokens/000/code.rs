// Answer 0

#[test]
fn test_to_tokens() {
    use quote::ToTokens;
    use proc_macro2::{TokenStream, Literal};

    struct TestStruct(u128);

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u128_suffixed(self.0));
        }
    }

    let value = TestStruct(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    let expected_tokens: TokenStream = quote::quote! { 42u128 };
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

