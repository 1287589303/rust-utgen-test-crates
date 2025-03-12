// Answer 0

#[test]
fn test_into_token_stream_basic() {
    struct TestStruct;
    
    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { test });
        }
    }
    
    let test_instance = TestStruct;
    let result = test_instance.into_token_stream();
    let expected: TokenStream = quote::quote! { test }.into();
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn test_into_token_stream_empty() {
    struct EmptyStruct;
    
    impl ToTokens for EmptyStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
    }
    
    let empty_instance = EmptyStruct;
    let result = empty_instance.into_token_stream();
    let expected: TokenStream = TokenStream::new();
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn test_into_token_stream_with_ident() {
    struct IdentStruct {
        ident: Ident,
    }
    
    impl ToTokens for IdentStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { #self.ident });
        }
    }
    
    let ident_instance = IdentStruct {
        ident: Ident::new("my_ident", Span::call_site()),
    };
    let result = ident_instance.into_token_stream();
    let expected: TokenStream = quote::quote! { my_ident }.into();
    assert_eq!(result.to_string(), expected.to_string());
}

