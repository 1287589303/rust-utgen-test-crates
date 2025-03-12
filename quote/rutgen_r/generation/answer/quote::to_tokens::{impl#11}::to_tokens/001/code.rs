// Answer 0

#[test]
fn test_to_tokens_positive() {
    struct TestValue(i64);
    
    impl TestValue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i64_suffixed(self.0));
        }
    }

    use quote::quote; // Assuming the quote crate is in scope
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    let value = TestValue(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected_tokens = quote!(42i64); // Assuming expected format for tokens
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_negative() {
    struct TestValue(i64);
    
    impl TestValue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i64_suffixed(self.0));
        }
    }

    use quote::quote;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    let value = TestValue(-10);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected_tokens = quote!(-10i64);
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_zero() {
    struct TestValue(i64);
    
    impl TestValue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i64_suffixed(self.0));
        }
    }

    use quote::quote;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    let value = TestValue(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    let expected_tokens = quote!(0i64);
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

