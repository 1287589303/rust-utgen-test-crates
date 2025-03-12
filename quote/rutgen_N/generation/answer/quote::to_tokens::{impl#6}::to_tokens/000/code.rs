// Answer 0

#[test]
fn test_to_tokens() {
    use quote::quote;
    use proc_macro2::TokenStream;
    use proc_macro2::TokenTree;
    use proc_macro2::Literal;

    struct TestStruct {
        value: String,
    }

    impl TestStruct {
        fn new(value: &str) -> Self {
            TestStruct {
                value: value.to_string(),
            }
        }
    }

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(&self.value));
        }
    }

    let test_value = "test string";
    let test_struct = TestStruct::new(test_value);
    let mut tokens = TokenStream::new();

    test_struct.to_tokens(&mut tokens);

    let expected_literal = Literal::string(test_value).to_string();
    let generated_tokens = tokens.to_string();

    assert!(generated_tokens.contains(&expected_literal));
}

#[test]
fn test_to_tokens_empty_string() {
    use quote::quote;
    use proc_macro2::TokenStream;
    use proc_macro2::TokenTree;
    use proc_macro2::Literal;

    struct TestStruct {
        value: String,
    }

    impl TestStruct {
        fn new(value: &str) -> Self {
            TestStruct {
                value: value.to_string(),
            }
        }
    }

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(&self.value));
        }
    }

    let test_value = "";
    let test_struct = TestStruct::new(test_value);
    let mut tokens = TokenStream::new();

    test_struct.to_tokens(&mut tokens);

    let expected_literal = Literal::string(test_value).to_string();
    let generated_tokens = tokens.to_string();

    assert!(generated_tokens.contains(&expected_literal));
}

