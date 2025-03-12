// Answer 0

#[test]
fn test_to_tokens_with_positive_i16() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct PositiveI16(i16);

    impl ToTokens for PositiveI16 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i16_suffixed(*self.0));
        }
    }

    let value = PositiveI16(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected = "42i16"; // Expected string representation
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_with_negative_i16() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct NegativeI16(i16);

    impl ToTokens for NegativeI16 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i16_suffixed(*self.0));
        }
    }

    let value = NegativeI16(-42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    let expected = "-42i16"; // Expected string representation
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_with_zero_i16() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct ZeroI16(i16);

    impl ToTokens for ZeroI16 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i16_suffixed(*self.0));
        }
    }

    let value = ZeroI16(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    let expected = "0i16"; // Expected string representation
    assert_eq!(tokens.to_string(), expected);
}

