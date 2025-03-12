// Answer 0

#[test]
fn test_to_tokens() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct FloatWrapper(f32);

    impl ToTokens for FloatWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f32_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let float = FloatWrapper(3.14);
    float.to_tokens(&mut tokens);

    let expected = "3.14_f32";
    assert_eq!(tokens.to_string(), expected);
}

