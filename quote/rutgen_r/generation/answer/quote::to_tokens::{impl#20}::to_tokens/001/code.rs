// Answer 0

#[test]
fn test_to_tokens_f32() {
    use proc_macro2::TokenStream;
    use quote::Literal;

    struct TestStruct(f32);

    let test_value = TestStruct(1.5);
    let mut tokens = TokenStream::new();
    
    test_value.to_tokens(&mut tokens);

    let expected_tokens = TokenStream::from(Literal::f32_suffixed(1.5));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_f32_zero() {
    use proc_macro2::TokenStream;
    use quote::Literal;

    struct TestStruct(f32);

    let test_value = TestStruct(0.0);
    let mut tokens = TokenStream::new();
    
    test_value.to_tokens(&mut tokens);

    let expected_tokens = TokenStream::from(Literal::f32_suffixed(0.0));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_f32_negative() {
    use proc_macro2::TokenStream;
    use quote::Literal;

    struct TestStruct(f32);

    let test_value = TestStruct(-2.5);
    let mut tokens = TokenStream::new();
    
    test_value.to_tokens(&mut tokens);

    let expected_tokens = TokenStream::from(Literal::f32_suffixed(-2.5));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

