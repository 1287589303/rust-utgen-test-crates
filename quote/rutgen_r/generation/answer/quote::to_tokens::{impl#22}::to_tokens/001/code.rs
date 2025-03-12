// Answer 0

#[test]
fn test_to_tokens_with_character() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct CharWrapper(char);
    
    impl ToTokens for CharWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::character(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let char_instance = CharWrapper('a');
    
    char_instance.to_tokens(&mut tokens);
    
    let expected = format!("'a'");
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_with_boundary_character() {
    use quote::ToTokens;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct CharWrapper(char);
    
    impl ToTokens for CharWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::character(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let char_instance = CharWrapper('\u{0000}'); // testing boundary with null character
    
    char_instance.to_tokens(&mut tokens);
    
    let expected = format!("'\\0'"); 
    assert_eq!(tokens.to_string(), expected);
}

