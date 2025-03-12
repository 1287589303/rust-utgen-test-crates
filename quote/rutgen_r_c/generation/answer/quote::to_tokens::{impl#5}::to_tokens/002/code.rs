// Answer 0

#[test]
fn test_to_tokens_with_some_bool_true() {
    use proc_macro2::TokenStream;
    use proc_macro2::Ident;

    struct SomeBool {
        value: bool,
    }
    
    impl std::ops::Deref for SomeBool {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    let some_true = SomeBool { value: true };
    let mut tokens = TokenStream::new();
    some_true.to_tokens(&mut tokens);
    
    let expected = TokenStream::from(Ident::new("true", Span::call_site()));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_some_bool_false() {
    use proc_macro2::TokenStream;
    use proc_macro2::Ident;

    struct SomeBool {
        value: bool,
    }
    
    impl std::ops::Deref for SomeBool {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    let some_false = SomeBool { value: false };
    let mut tokens = TokenStream::new();
    some_false.to_tokens(&mut tokens);
    
    let expected = TokenStream::from(Ident::new("false", Span::call_site()));
    assert_eq!(tokens.to_string(), expected.to_string());
}

