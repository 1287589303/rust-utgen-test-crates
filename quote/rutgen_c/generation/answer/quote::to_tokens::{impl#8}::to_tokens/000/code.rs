// Answer 0

#[test]
fn test_to_tokens_with_true() {
    struct TrueLiteral(i8);
    
    impl TrueLiteral {
        fn new(value: i8) -> Self {
            TrueLiteral(value)
        }
    }

    impl ToTokens for TrueLiteral {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i8_suffixed(self.0));
        }
    }

    let value = TrueLiteral::new(1);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "1i8");
}

#[test]
fn test_to_tokens_with_false() {
    struct FalseLiteral(i8);

    impl FalseLiteral {
        fn new(value: i8) -> Self {
            FalseLiteral(value)
        }
    }

    impl ToTokens for FalseLiteral {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i8_suffixed(self.0));
        }
    }

    let value = FalseLiteral::new(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "0i8");
}

