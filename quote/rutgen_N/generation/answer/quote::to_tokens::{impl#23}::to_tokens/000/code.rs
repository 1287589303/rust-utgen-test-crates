// Answer 0

#[derive(Debug)]
struct BoolWrapper(bool);

impl std::ops::Deref for BoolWrapper {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_to_tokens_true() {
    let mut tokens = TokenStream::new();
    let wrapper = BoolWrapper(true);
    wrapper.to_tokens(&mut tokens);
    
    let expected_ident = Ident::new("true", Span::call_site());
    assert!(tokens.to_string().contains(&expected_ident.to_string()));
}

#[test]
fn test_to_tokens_false() {
    let mut tokens = TokenStream::new();
    let wrapper = BoolWrapper(false);
    wrapper.to_tokens(&mut tokens);
    
    let expected_ident = Ident::new("false", Span::call_site());
    assert!(tokens.to_string().contains(&expected_ident.to_string()));
}

