// Answer 0

#[test]
fn test_to_tokens_positive_integer() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestInt(i32);
    
    impl ToTokens for TestInt {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(self.0));
        }
    }

    let test_value = TestInt(42);
    let mut tokens = TokenStream::new();
    test_value.to_tokens(&mut tokens);

    let expected = Literal::i32_suffixed(42).to_string();
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_negative_integer() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestInt(i32);
    
    impl ToTokens for TestInt {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(self.0));
        }
    }

    let test_value = TestInt(-10);
    let mut tokens = TokenStream::new();
    test_value.to_tokens(&mut tokens);

    let expected = Literal::i32_suffixed(-10).to_string();
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_zero() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestInt(i32);
    
    impl ToTokens for TestInt {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(self.0));
        }
    }

    let test_value = TestInt(0);
    let mut tokens = TokenStream::new();
    test_value.to_tokens(&mut tokens);

    let expected = Literal::i32_suffixed(0).to_string();
    assert_eq!(tokens.to_string(), expected);
}

