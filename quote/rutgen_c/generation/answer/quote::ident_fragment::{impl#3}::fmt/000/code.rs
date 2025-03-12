// Answer 0

#[test]
fn test_fmt_with_ident_fragment() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestIdentFragment")
        }
    }

    struct TestOwnedIdentFragment<'a>(Cow<'a, TestIdentFragment>);

    impl ToTokens for TestOwnedIdentFragment<'_> {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream {
            TokenStream::new()
        }
        fn into_token_stream(self) -> TokenStream {
            TokenStream::new()
        }
    }

    let test_fragment = TestOwnedIdentFragment(Cow::Owned(TestIdentFragment));
    let mut output = String::new();
    let result = test_fragment.0.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "TestIdentFragment");
}

#[test]
fn test_fmt_with_cow_ident_fragment() {
    struct CowIdentFragment;

    impl IdentFragment for CowIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "CowIdentFragment")
        }
    }

    let cow_fragment: Cow<CowIdentFragment> = Cow::Owned(CowIdentFragment);
    let mut output = String::new();
    let result = cow_fragment.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "CowIdentFragment");
}

