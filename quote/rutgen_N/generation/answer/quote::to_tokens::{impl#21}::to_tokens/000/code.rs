// Answer 0

#[test]
fn test_to_tokens_with_valid_value() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct(f64);

    let value = TestStruct(3.14);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    assert_eq!(tokens.to_string(), "3.14f64");
}

#[test]
#[should_panic]
fn test_to_tokens_with_invalid_value() {
    use quote::TokenStream;
    use quote::Literal;

    struct TestStruct(f64);

    let value = TestStruct(f64::NAN);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    // Assuming that appending an invalid f64 value will panic
    assert!(tokens.to_string() == "NANf64"); // This will trigger a panic if the condition is not met
}

