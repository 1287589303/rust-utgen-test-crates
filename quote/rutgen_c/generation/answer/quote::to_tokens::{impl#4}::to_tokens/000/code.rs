// Answer 0

#[test]
fn test_to_tokens_with_rc() {
    struct TestToken;

    impl ToTokens for TestToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let ident = Ident::new("test", Span::call_site());
            tokens.extend(iter::once(TokenTree::Ident(ident)));
        }
    }

    let mut tokens = TokenStream::new();
    let test_token = TestToken;
    let rc_token = Rc::new(test_token);
    rc_token.to_tokens(&mut tokens);

    let expected_token = TokenStream::from(TokenTree::Ident(Ident::new("test", Span::call_site())));
    assert_eq!(tokens.to_string(), expected_token.to_string());
}

#[test]
fn test_to_token_stream_with_rc() {
    struct TestToken;

    impl ToTokens for TestToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let lit = Literal::u32_unsuffixed(42);
            tokens.extend(iter::once(TokenTree::Literal(lit)));
        }
    }

    let test_token = TestToken;
    let rc_token = Rc::new(test_token);
    let tokens = rc_token.to_token_stream();

    let expected_token = TokenStream::from(TokenTree::Literal(Literal::u32_unsuffixed(42)));
    assert_eq!(tokens.to_string(), expected_token.to_string());
}

#[test]
fn test_into_token_stream_with_rc() {
    struct TestToken;

    impl ToTokens for TestToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let punct = Punct::new(';', proc_macro2::Spacing::Alone);
            tokens.extend(iter::once(TokenTree::Punct(punct)));
        }
    }

    let test_token = TestToken;
    let rc_token = Rc::new(test_token);
    let tokens = rc_token.clone().into_token_stream();

    let expected_token = TokenStream::from(TokenTree::Punct(Punct::new(';', proc_macro2::Spacing::Alone)));
    assert_eq!(tokens.to_string(), expected_token.to_string());
}

