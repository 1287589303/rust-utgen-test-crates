// Answer 0

#[test]
fn test_to_tokens_float_literal() {
    use proc_macro2::TokenStream;
    use quote::ToTokens;

    struct TestFloat(f64);

    impl ToTokens for TestFloat {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f64_suffixed(self.0));
        }
    }

    let float_value = TestFloat(3.14);
    let mut tokens = TokenStream::new();
    float_value.to_tokens(&mut tokens);

    let expected: TokenStream = quote::quote! { 3.14_f64 };
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_zero_float_literal() {
    use proc_macro2::TokenStream;
    use quote::ToTokens;

    struct ZeroFloat;

    impl ToTokens for ZeroFloat {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f64_suffixed(0.0));
        }
    }

    let zero_value = ZeroFloat;
    let mut tokens = TokenStream::new();
    zero_value.to_tokens(&mut tokens);

    let expected: TokenStream = quote::quote! { 0.0_f64 };
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_negative_float_literal() {
    use proc_macro2::TokenStream;
    use quote::ToTokens;

    struct NegativeFloat(f64);

    impl ToTokens for NegativeFloat {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f64_suffixed(self.0));
        }
    }

    let negative_value = NegativeFloat(-1.23);
    let mut tokens = TokenStream::new();
    negative_value.to_tokens(&mut tokens);

    let expected: TokenStream = quote::quote! { -1.23_f64 };
    assert_eq!(tokens.to_string(), expected.to_string());
}

