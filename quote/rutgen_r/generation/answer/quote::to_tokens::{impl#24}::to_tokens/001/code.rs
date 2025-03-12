// Answer 0

#[test]
fn test_to_tokens_with_valid_literal() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::c_string(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_struct = TestStruct {
        value: "Hello, world!",
    };
    
    test_struct.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), r#""Hello, world!""#);
}

#[test]
fn test_to_tokens_with_empty_literal() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::c_string(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_struct = TestStruct {
        value: "",
    };
    
    test_struct.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), r#""""#);
}

#[should_panic]
fn test_to_tokens_with_null() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct {
        value: Option<&'static str>,
    }

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            if let Some(val) = self.value {
                tokens.append(Literal::c_string(val));
            } else {
                panic!("Value is null");
            }
        }
    }

    let mut tokens = TokenStream::new();
    let test_struct = TestStruct {
        value: None,
    };

    test_struct.to_tokens(&mut tokens);
}

