// Answer 0

#[test]
fn test_to_tokens_with_positive_i128() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct PositiveI128(i128);

    impl ToTokens for PositiveI128 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i128_suffixed(self.0));
        }
    }

    let value = PositiveI128(1234567890123456789);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "1234567890123456789_i128");
}

#[test]
fn test_to_tokens_with_zero() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct ZeroI128(i128);

    impl ToTokens for ZeroI128 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i128_suffixed(self.0));
        }
    }

    let value = ZeroI128(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "0_i128");
}

#[test]
fn test_to_tokens_with_negative_i128() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct NegativeI128(i128);

    impl ToTokens for NegativeI128 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i128_suffixed(self.0));
        }
    }

    let value = NegativeI128(-1234567890123456789);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "-1234567890123456789_i128");
}

