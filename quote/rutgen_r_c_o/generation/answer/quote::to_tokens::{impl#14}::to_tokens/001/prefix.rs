// Answer 0

#[test]
fn test_to_tokens_zero() {
    struct TestU8(u8);
    
    let mut tokens = TokenStream::new();
    TestU8(0).to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_one() {
    struct TestU8(u8);
    
    let mut tokens = TokenStream::new();
    TestU8(1).to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_mid_value() {
    struct TestU8(u8);
    
    let mut tokens = TokenStream::new();
    TestU8(128).to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_two_fifty_four() {
    struct TestU8(u8);
    
    let mut tokens = TokenStream::new();
    TestU8(254).to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_two_fifty_five() {
    struct TestU8(u8);
    
    let mut tokens = TokenStream::new();
    TestU8(255).to_tokens(&mut tokens);
}

