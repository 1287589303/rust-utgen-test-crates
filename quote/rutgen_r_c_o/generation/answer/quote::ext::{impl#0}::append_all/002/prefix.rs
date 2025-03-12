// Answer 0

#[test]
fn test_append_all_single_token() {
    struct Token1;
    
    impl ToTokens for Token1 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { token1 });
        }
        
        fn to_token_stream(&self) -> TokenStream {
            let mut ts = TokenStream::new();
            self.to_tokens(&mut ts);
            ts
        }
        
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    let mut ts = TokenStream::new();
    let tokens = vec![Token1];
    ts.append_all(tokens);
}

#[test]
fn test_append_all_multiple_tokens() {
    struct Token2;
    
    impl ToTokens for Token2 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { token2 });
        }
        
        fn to_token_stream(&self) -> TokenStream {
            let mut ts = TokenStream::new();
            self.to_tokens(&mut ts);
            ts
        }
        
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    let mut ts = TokenStream::new();
    let tokens = vec![Token2, Token2];
    ts.append_all(tokens);
}

#[test]
fn test_append_all_boundary_case() {
    struct Token3;
    
    impl ToTokens for Token3 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { token3 });
        }
        
        fn to_token_stream(&self) -> TokenStream {
            let mut ts = TokenStream::new();
            self.to_tokens(&mut ts);
            ts
        }
        
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    let mut ts = TokenStream::new();
    let tokens = vec![Token3];
    ts.append_all(tokens);
}

