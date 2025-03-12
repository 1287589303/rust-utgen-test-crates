// Answer 0

#[test]
fn test_to_tokens() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;

    struct MyStruct {
        value: String,
    }

    impl MyStruct {
        fn as_str(&self) -> &str {
            &self.value
        }
    }

    impl ToTokens for MyStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.as_str().to_tokens(tokens);
        }
    }

    let my_struct = MyStruct {
        value: String::from("test"),
    };

    let mut tokens = TokenStream::new();
    my_struct.to_tokens(&mut tokens);

    let expected_tokens: TokenStream = "test".parse().unwrap();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_empty() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;

    struct MyEmptyStruct {
        value: String,
    }

    impl MyEmptyStruct {
        fn as_str(&self) -> &str {
            &self.value
        }
    }

    impl ToTokens for MyEmptyStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.as_str().to_tokens(tokens);
        }
    }

    let my_empty_struct = MyEmptyStruct {
        value: String::from(""),
    };

    let mut tokens = TokenStream::new();
    my_empty_struct.to_tokens(&mut tokens);

    let expected_tokens: TokenStream = "".parse().unwrap();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

