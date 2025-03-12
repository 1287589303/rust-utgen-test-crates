// Answer 0

#[test]
fn test_to_tokens() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct {
        value: i16,
    }

    impl TestStruct {
        fn new(value: i16) -> Self {
            TestStruct { value }
        }

        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i16_suffixed(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_value = 42_i16;
    let test_struct = TestStruct::new(test_value);

    test_struct.to_tokens(&mut tokens);
    
    // Assuming tokens now contains the correct representation    
    assert!(tokens.to_string().contains("42"));
}

#[test]
fn test_to_tokens_negative_value() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct {
        value: i16,
    }

    impl TestStruct {
        fn new(value: i16) -> Self {
            TestStruct { value }
        }

        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i16_suffixed(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_value = -100_i16;
    let test_struct = TestStruct::new(test_value);

    test_struct.to_tokens(&mut tokens);
    
    // Assuming tokens now contains the correct representation    
    assert!(tokens.to_string().contains("-100"));
}

