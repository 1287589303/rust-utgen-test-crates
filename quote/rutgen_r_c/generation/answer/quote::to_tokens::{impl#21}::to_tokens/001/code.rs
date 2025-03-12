// Answer 0

#[test]
fn test_to_tokens_float() {
    use proc_macro2::{TokenStream, Literal};

    struct FloatWrapper(f64);

    impl ToTokens for FloatWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f64_suffixed(self.0));
        }
    }

    let float_value = FloatWrapper(3.14);
    let mut tokens = TokenStream::new();
    float_value.to_tokens(&mut tokens);
    
    let expected_token = Literal::f64_suffixed(3.14).to_string();
    assert_eq!(tokens.to_string(), expected_token);
}

#[test]
fn test_to_tokens_zero_float() {
    use proc_macro2::{TokenStream, Literal};

    struct FloatWrapper(f64);

    impl ToTokens for FloatWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f64_suffixed(self.0));
        }
    }

    let float_value = FloatWrapper(0.0);
    let mut tokens = TokenStream::new();
    float_value.to_tokens(&mut tokens);
    
    let expected_token = Literal::f64_suffixed(0.0).to_string();
    assert_eq!(tokens.to_string(), expected_token);
}

#[test]
fn test_to_tokens_negative_float() {
    use proc_macro2::{TokenStream, Literal};

    struct FloatWrapper(f64);

    impl ToTokens for FloatWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::f64_suffixed(self.0));
        }
    }

    let float_value = FloatWrapper(-1.5);
    let mut tokens = TokenStream::new();
    float_value.to_tokens(&mut tokens);
    
    let expected_token = Literal::f64_suffixed(-1.5).to_string();
    assert_eq!(tokens.to_string(), expected_token);
}

