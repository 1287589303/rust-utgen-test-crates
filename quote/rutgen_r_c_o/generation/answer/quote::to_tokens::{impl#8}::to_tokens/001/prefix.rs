// Answer 0

#[test]
fn test_to_tokens_with_minimum_i8() {
    struct TestStruct(i8);
    let value = TestStruct(-128);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_with_zero_i8() {
    struct TestStruct(i8);
    let value = TestStruct(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_with_maximum_i8() {
    struct TestStruct(i8);
    let value = TestStruct(127);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
#[should_panic]
fn test_to_tokens_with_below_min_i8() {
    struct TestStruct(i8);
    let value = TestStruct(-129);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
#[should_panic]
fn test_to_tokens_with_above_max_i8() {
    struct TestStruct(i8);
    let value = TestStruct(128);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

