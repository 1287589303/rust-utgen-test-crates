// Answer 0

#[test]
fn test_to_tokens_zero() {
    let value: u16 = 0;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_one() {
    let value: u16 = 1;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_sixty_four() {
    let value: u16 = 64;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_sixty_five_thousand_five_hundred_thirty_five() {
    let value: u16 = 65535;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

