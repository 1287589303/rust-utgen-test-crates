// Answer 0

#[test]
fn test_to_tokens_true() {
    use proc_macro2::{TokenStream, Literal};

    struct BoolWrapper(bool);

    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::usize_suffixed(self.0 as usize));
        }
    }

    let mut tokens = TokenStream::new();
    let wrapper = BoolWrapper(true);
    wrapper.to_tokens(&mut tokens);
    let expected = Literal::usize_suffixed(1).to_string();
    assert_eq!(tokens.to_string(), expected);
}

#[test]
fn test_to_tokens_false() {
    use proc_macro2::{TokenStream, Literal};

    struct BoolWrapper(bool);

    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::usize_suffixed(self.0 as usize));
        }
    }

    let mut tokens = TokenStream::new();
    let wrapper = BoolWrapper(false);
    wrapper.to_tokens(&mut tokens);
    let expected = Literal::usize_suffixed(0).to_string();
    assert_eq!(tokens.to_string(), expected);
}

