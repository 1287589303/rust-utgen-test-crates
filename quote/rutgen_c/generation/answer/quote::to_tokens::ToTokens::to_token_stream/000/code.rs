// Answer 0

#[test]
fn test_to_token_stream_basic() {
    use proc_macro2::{TokenStream, TokenTree, Ident};

    struct Example;
    
    impl ToTokens for Example {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { let x = 10; });
        }
        
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    let example = Example;
    let token_stream = example.to_token_stream();
    let expected: TokenStream = quote::quote! { let x = 10; };
    
    assert_eq!(token_stream.to_string(), expected.to_string());
}

#[test]
fn test_to_token_stream_empty() {
    use proc_macro2::{TokenStream, TokenTree};

    struct Empty;
    
    impl ToTokens for Empty {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            // No tokens are added
        }
        
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    let empty = Empty;
    let token_stream = empty.to_token_stream();
    
    assert!(token_stream.is_empty());
}

#[test]
fn test_to_token_stream_complex() {
    use proc_macro2::{TokenStream, TokenTree, Literal};

    struct Complex;
    
    impl ToTokens for Complex {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { fn example() { let y = #Lit(42); } });
        }
        
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    let complex = Complex;
    let token_stream = complex.to_token_stream();
    let expected: TokenStream = quote::quote! { fn example() { let y = 42; } };
    
    assert_eq!(token_stream.to_string(), expected.to_string());
}

