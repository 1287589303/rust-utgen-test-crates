// Answer 0

#[test]
fn test_to_tokens() {
    use quote::{ToTokens, Literal};
    use proc_macro2::TokenStream;

    struct TestStruct {
        value: &'static str,
    }

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::c_string(self.value));
        }
    }

    let test_instance = TestStruct { value: "test" };
    let mut tokens = TokenStream::new();
    test_instance.to_tokens(&mut tokens);
    
    let expected_tokens: TokenStream = Literal::c_string("test").into();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

