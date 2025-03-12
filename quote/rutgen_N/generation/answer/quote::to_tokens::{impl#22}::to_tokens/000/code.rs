// Answer 0

#[test]
fn test_to_tokens() {
    use quote::TokenStream;
    use quote::Literal;

    struct CharWrapper(char);

    impl CharWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::character(*self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let character = CharWrapper('a');
    character.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "'a'");
}

#[test]
fn test_to_tokens_empty() {
    use quote::TokenStream;
    use quote::Literal;

    struct CharWrapper(char);

    impl CharWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::character(*self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let character = CharWrapper(' ');
    character.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "' '");
}

