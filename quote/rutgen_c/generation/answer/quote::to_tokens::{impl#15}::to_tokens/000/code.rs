// Answer 0

#[test]
fn test_to_tokens_true() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;
    
    struct TestBool(bool);

    impl ToTokens for TestBool {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u16_suffixed(if self.0 { 1 } else { 0 }));
        }
    }

    let mut tokens = TokenStream::new();
    let test_bool = TestBool(true);
    test_bool.to_tokens(&mut tokens);

    let expected = Literal::u16_suffixed(1).to_string();
    assert!(tokens.to_string().contains(&expected));
}

#[test]
fn test_to_tokens_false() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestBool(bool);

    impl ToTokens for TestBool {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::u16_suffixed(if self.0 { 1 } else { 0 }));
        }
    }

    let mut tokens = TokenStream::new();
    let test_bool = TestBool(false);
    test_bool.to_tokens(&mut tokens);

    let expected = Literal::u16_suffixed(0).to_string();
    assert!(tokens.to_string().contains(&expected));
}

