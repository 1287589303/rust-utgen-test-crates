// Answer 0

#[test]
fn test_to_tokens_true() {
    let value: &bool = &true;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_false() {
    let value: &bool = &false;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

