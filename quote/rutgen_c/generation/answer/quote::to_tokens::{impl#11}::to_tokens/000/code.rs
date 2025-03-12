// Answer 0

#[test]
fn test_to_tokens_with_positive_i64() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestInt(i64);

    impl ToTokens for TestInt {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i64_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let value = TestInt(42);
    value.to_tokens(&mut tokens);
    
    let expected = Literal::i64_suffixed(42).to_string();
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_with_negative_i64() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestInt(i64);

    impl ToTokens for TestInt {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i64_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let value = TestInt(-15);
    value.to_tokens(&mut tokens);
    
    let expected = Literal::i64_suffixed(-15).to_string();
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_with_zero_i64() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestInt(i64);

    impl ToTokens for TestInt {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i64_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let value = TestInt(0);
    value.to_tokens(&mut tokens);
    
    let expected = Literal::i64_suffixed(0).to_string();
    assert_eq!(tokens.to_string(), expected);
}

