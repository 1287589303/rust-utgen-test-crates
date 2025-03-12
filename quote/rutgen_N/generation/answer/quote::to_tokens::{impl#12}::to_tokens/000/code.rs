// Answer 0

#[test]
fn test_to_tokens() {
    use quote::{TokenStream, Literal};

    struct TestStruct(i128);

    let test_value = TestStruct(42);
    let mut tokens = TokenStream::new();
    
    test_value.to_tokens(&mut tokens);

    let expected_token = Literal::i128_suffixed(42);
    assert_eq!(tokens.to_string(), expected_token.to_string());
}

