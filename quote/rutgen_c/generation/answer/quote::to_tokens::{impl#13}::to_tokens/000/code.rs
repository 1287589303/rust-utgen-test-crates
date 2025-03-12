// Answer 0

#[test]
fn test_to_tokens_with_positive_isize() {
    struct IsizeTest(u8);
    
    impl ToTokens for IsizeTest {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0 as isize));
        }
    }

    let value = IsizeTest(5);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected = Literal::isize_suffixed(5);
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_negative_isize() {
    struct IsizeTest(u8);
    
    impl ToTokens for IsizeTest {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(-(self.0 as isize)));
        }
    }

    let value = IsizeTest(3);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected = Literal::isize_suffixed(-3);
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_zero_isize() {
    struct IsizeTest(u8);
    
    impl ToTokens for IsizeTest {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(0));
        }
    }

    let value = IsizeTest(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected = Literal::isize_suffixed(0);
    assert_eq!(tokens.to_string(), expected.to_string());
}

