// Answer 0

#[test]
fn test_to_tokens_with_zero() {
    use proc_macro2::TokenStream;

    struct TestIsize(i32);

    impl ToTokens for TestIsize {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0 as isize));
        }
    }

    let test_value = TestIsize(0);
    let mut tokens = TokenStream::new();
    test_value.to_tokens(&mut tokens);
    let expected_tokens = TokenStream::from(Literal::isize_suffixed(0));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_with_positive_value() {
    use proc_macro2::TokenStream;

    struct TestIsize(i32);

    impl ToTokens for TestIsize {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0 as isize));
        }
    }

    let test_value = TestIsize(42);
    let mut tokens = TokenStream::new();
    test_value.to_tokens(&mut tokens);
    let expected_tokens = TokenStream::from(Literal::isize_suffixed(42));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_with_negative_value() {
    use proc_macro2::TokenStream;

    struct TestIsize(i32);

    impl ToTokens for TestIsize {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0 as isize));
        }
    }

    let test_value = TestIsize(-23);
    let mut tokens = TokenStream::new();
    test_value.to_tokens(&mut tokens);
    let expected_tokens = TokenStream::from(Literal::isize_suffixed(-23));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

