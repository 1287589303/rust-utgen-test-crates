// Answer 0

#[test]
fn test_to_tokens() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct {
        value: isize,
    }

    impl TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.value));
        }
    }

    let mut tokens = TokenStream::new();
    let test_instance = TestStruct { value: 42 };
    
    test_instance.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "42i");
}

