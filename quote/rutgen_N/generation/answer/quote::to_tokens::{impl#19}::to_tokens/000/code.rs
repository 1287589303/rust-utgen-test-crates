// Answer 0

#[test]
fn test_to_tokens() {
    use quote::quote;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct MyStruct(usize);

    impl MyStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::usize_suffixed(self.0));
        }
    }

    let my_instance = MyStruct(42);
    let mut tokens = TokenStream::new();
    my_instance.to_tokens(&mut tokens);

    let expected_tokens: TokenStream = quote! { 42 };
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

