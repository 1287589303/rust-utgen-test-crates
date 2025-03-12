// Answer 0

#[test]
fn test_to_tokens_zero() {
    struct TestU128(u128);
    let mut tokens = TokenStream::new();
    TestU128(0).to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_one() {
    struct TestU128(u128);
    let mut tokens = TokenStream::new();
    TestU128(1).to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_max_value() {
    struct TestU128(u128);
    let mut tokens = TokenStream::new();
    TestU128(u128::MAX).to_tokens(&mut tokens);
}

#[should_panic]
#[test]
fn test_to_tokens_negative_one() {
    struct TestU128(i128);
    let mut tokens = TokenStream::new();
    TestU128(-1).to_tokens(&mut tokens);
}

#[should_panic]
#[test]
fn test_to_tokens_negative_two() {
    struct TestU128(i128);
    let mut tokens = TokenStream::new();
    TestU128(-2).to_tokens(&mut tokens);
}

