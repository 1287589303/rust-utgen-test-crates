// Answer 0

#[test]
fn test_to_tokens_u8() {
    use proc_macro2::TokenStream;

    struct U8Wrapper(u8);

    impl ToTokens for U8Wrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u8_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let value = U8Wrapper(10);
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::u8_suffixed(10));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_u8_zero() {
    use proc_macro2::TokenStream;

    struct U8Wrapper(u8);

    impl ToTokens for U8Wrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u8_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let value = U8Wrapper(0);
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::u8_suffixed(0));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_u8_max() {
    use proc_macro2::TokenStream;

    struct U8Wrapper(u8);

    impl ToTokens for U8Wrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u8_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let value = U8Wrapper(255);
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::u8_suffixed(255));
    assert_eq!(tokens.to_string(), expected.to_string());
}

