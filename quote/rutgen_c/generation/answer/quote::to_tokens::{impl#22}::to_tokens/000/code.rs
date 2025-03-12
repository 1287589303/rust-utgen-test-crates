// Answer 0

#[test]
fn test_to_tokens_true() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestBool(bool);

    impl ToTokens for TestBool {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::character(self.0 as char));
        }
    }

    let test_value = TestBool(true);
    let mut tokens = TokenStream::new();
    test_value.to_tokens(&mut tokens);
    
    let expected = Literal::character('t').to_string(); 
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_false() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestBool(bool);

    impl ToTokens for TestBool {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::character(self.0 as char));
        }
    }

    let test_value = TestBool(false);
    let mut tokens = TokenStream::new();
    test_value.to_tokens(&mut tokens);

    let expected = Literal::character('f').to_string(); 
    assert_eq!(tokens.to_string(), expected);
}

