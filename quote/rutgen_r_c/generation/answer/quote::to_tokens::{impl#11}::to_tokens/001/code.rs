// Answer 0

#[test]
fn test_to_tokens_with_positive_integer() {
    struct TestStruct(i64);
    
    let value = TestStruct(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "42i64");
}

#[test]
fn test_to_tokens_with_negative_integer() {
    struct TestStruct(i64);
    
    let value = TestStruct(-10);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "-10i64");
}

#[test]
fn test_to_tokens_with_zero() {
    struct TestStruct(i64);
    
    let value = TestStruct(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "0i64");
}

#[test]
fn test_to_tokens_with_large_integer() {
    struct TestStruct(i64);
    
    let value = TestStruct(1_000_000);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "1000000i64");
}

