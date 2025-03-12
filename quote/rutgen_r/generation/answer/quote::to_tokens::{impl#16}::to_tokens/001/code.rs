// Answer 0

#[test]
fn test_to_tokens_with_zero() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct Num(u32);

    impl ToTokens for Num {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u32_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let num = Num(0);
    num.to_tokens(&mut tokens);
    let expected = r"0u32"; // this is a simplification; the actual TokenStream debug representation can vary
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_with_positive_number() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct Num(u32);

    impl ToTokens for Num {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u32_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let num = Num(42);
    num.to_tokens(&mut tokens);
    let expected = r"42u32"; // this is a simplification; the actual TokenStream debug representation can vary
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_with_max_u32() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct Num(u32);

    impl ToTokens for Num {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u32_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let num = Num(u32::MAX);
    num.to_tokens(&mut tokens);
    let expected = format!("{}u32", u32::MAX);
    assert_eq!(tokens.to_string(), expected);
}

