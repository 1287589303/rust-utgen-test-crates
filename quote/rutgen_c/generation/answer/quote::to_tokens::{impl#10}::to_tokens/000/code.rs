// Answer 0

#[test]
fn test_to_tokens_with_positive_integer() {
    struct TestInteger(i32);
    
    impl ToTokens for TestInteger {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(self.0));
        }
    }

    let value = TestInteger(5);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::i32_suffixed(5));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_negative_integer() {
    struct TestInteger(i32);
    
    impl ToTokens for TestInteger {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(self.0));
        }
    }

    let value = TestInteger(-3);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::i32_suffixed(-3));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_zero() {
    struct TestInteger(i32);
    
    impl ToTokens for TestInteger {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(self.0));
        }
    }

    let value = TestInteger(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected = TokenStream::from(Literal::i32_suffixed(0));
    assert_eq!(tokens.to_string(), expected.to_string());
}

