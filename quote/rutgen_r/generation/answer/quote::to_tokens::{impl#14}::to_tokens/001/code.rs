// Answer 0

#[test]
fn test_to_tokens_with_small_value() {
    struct TestStruct {
        value: u8,
    }
    
    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u8_suffixed(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_value = TestStruct { value: 5 };
    test_value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "5u8");
}

#[test]
fn test_to_tokens_with_zero() {
    struct TestStruct {
        value: u8,
    }
    
    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u8_suffixed(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_value = TestStruct { value: 0 };
    test_value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "0u8");
}

#[test]
fn test_to_tokens_with_max_value() {
    struct TestStruct {
        value: u8,
    }
    
    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u8_suffixed(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_value = TestStruct { value: 255 };
    test_value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "255u8");
}

