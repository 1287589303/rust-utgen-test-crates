// Answer 0

#[test]
fn test_to_tokens_false() {
    use quote::TokenStream;
    use proc_macro2::Ident;
    use proc_macro2::Span;

    struct FalseFlag;

    impl core::ops::Deref for FalseFlag {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &false
        }
    }

    let mut tokens = TokenStream::new();
    let false_flag = FalseFlag;

    false_flag.to_tokens(&mut tokens);

    let expected_token = Ident::new("false", Span::call_site());
    assert!(tokens.to_string().contains(&expected_token.to_string()));
}

