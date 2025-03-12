// Answer 0

#[test]
fn test_to_tokens_bool_true() {
    use proc_macro2::{TokenStream, Ident, Span};
    
    struct BoolWrapper(bool);
    
    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let word = if self.0 { "true" } else { "false" };
            tokens.append(Ident::new(word, Span::call_site()));
        }
    }
    
    let true_value = BoolWrapper(true);
    let mut tokens = TokenStream::new();
    true_value.to_tokens(&mut tokens);
    
    let expected: TokenStream = Ident::new("true", Span::call_site()).into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_bool_false() {
    use proc_macro2::{TokenStream, Ident, Span};
    
    struct BoolWrapper(bool);
    
    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let word = if self.0 { "true" } else { "false" };
            tokens.append(Ident::new(word, Span::call_site()));
        }
    }

    let false_value = BoolWrapper(false);
    let mut tokens = TokenStream::new();
    false_value.to_tokens(&mut tokens);
    
    let expected: TokenStream = Ident::new("false", Span::call_site()).into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

