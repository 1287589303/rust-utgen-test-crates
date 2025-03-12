// Answer 0

#[test]
fn test_cow_to_tokens_with_ident() {
    struct TestIdent;

    impl ToTokens for TestIdent {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let ident = Ident::new("test_ident", Span::call_site());
            tokens.extend(iter::once(TokenTree::Ident(ident)));
        }
    }

    let test_ident = TestIdent;
    let cow = Cow::Owned(test_ident);
    let mut tokens = TokenStream::new();
    cow.to_tokens(&mut tokens);
    let expected = quote::quote! { test_ident };
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_cow_to_tokens_with_literal() {
    struct TestLiteral;

    impl ToTokens for TestLiteral {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let literal = Literal::usize_unsuffixed(42);
            tokens.extend(iter::once(TokenTree::Literal(literal)));
        }
    }

    let test_literal = TestLiteral;
    let cow = Cow::Owned(test_literal);
    let mut tokens = TokenStream::new();
    cow.to_tokens(&mut tokens);
    let expected = quote::quote! { 42 };
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_cow_to_tokens_with_group() {
    struct TestGroup;

    impl ToTokens for TestGroup {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let group = Group::new(Span::call_site(), TokenStream::new());
            tokens.extend(iter::once(TokenTree::Group(group)));
        }
    }

    let test_group = TestGroup;
    let cow = Cow::Owned(test_group);
    let mut tokens = TokenStream::new();
    cow.to_tokens(&mut tokens);
    let expected = quote::quote! { {} };
    assert_eq!(tokens.to_string(), expected.to_string());
}

