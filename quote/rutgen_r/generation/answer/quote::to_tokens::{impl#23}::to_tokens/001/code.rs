// Answer 0

#[cfg(test)]
fn test_to_tokens_true() {
    use quote::ToTokens; // Replace with the correct import path for ToTokens
    use proc_macro2::{TokenStream, Ident, Span};

    struct TrueBool;
    impl ToTokens for TrueBool {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let word = if *self { "true" } else { "false" };
            tokens.append(Ident::new(word, Span::call_site()));
        }
    }

    let val = TrueBool;
    let mut tokens = TokenStream::new();
    val.to_tokens(&mut tokens);
    
    let expected = "true"; // Expected output for tokens in string format
    assert_eq!(tokens.to_string(), expected);
}

#[cfg(test)]
fn test_to_tokens_false() {
    use quote::ToTokens; // Replace with the correct import path for ToTokens
    use proc_macro2::{TokenStream, Ident, Span};

    struct FalseBool;
    impl ToTokens for FalseBool {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let word = if *self { "true" } else { "false" };
            tokens.append(Ident::new(word, Span::call_site()));
        }
    }

    let val = FalseBool;
    let mut tokens = TokenStream::new();
    val.to_tokens(&mut tokens);
    
    let expected = "false"; // Expected output for tokens in string format
    assert_eq!(tokens.to_string(), expected);
}

