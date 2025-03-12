// Answer 0

#[test]
fn test_to_tokens_true() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct BoolWrapper(bool);

    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::character(self.0 as char));
        }
    }

    let mut tokens = TokenStream::new();
    let value = BoolWrapper(true);
    value.to_tokens(&mut tokens);
    
    let expected: TokenStream = Literal::character('1').into(); // As `true` is expected to be '1'
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_false() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct BoolWrapper(bool);

    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::character(self.0 as char));
        }
    }

    let mut tokens = TokenStream::new();
    let value = BoolWrapper(false);
    value.to_tokens(&mut tokens);
    
    let expected: TokenStream = Literal::character('0').into(); // As `false` is expected to be '0'
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_boundary() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct BoolWrapper(bool);

    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::character(self.0 as char));
        }
    }

    let mut tokens = TokenStream::new();
    let value_true = BoolWrapper(true);
    let value_false = BoolWrapper(false);
    
    value_true.to_tokens(&mut tokens);
    value_false.to_tokens(&mut tokens);
    
    let expected: TokenStream = Literal::character('1').into().append(Literal::character('0'));
    assert_eq!(tokens.to_string(), expected.to_string());
}

