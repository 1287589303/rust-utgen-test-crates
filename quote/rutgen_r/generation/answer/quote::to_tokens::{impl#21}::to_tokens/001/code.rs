// Answer 0

#[test]
fn test_to_tokens_with_valid_f64() {
    use quote::TokenStream;
    use quote::Literal;

    struct MyStruct(f64);

    let value = MyStruct(3.14);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    assert_eq!(tokens.to_string(), "3.14f64");
}

#[test]
fn test_to_tokens_with_zero() {
    use quote::TokenStream;
    use quote::Literal;

    struct MyStruct(f64);

    let value = MyStruct(0.0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    assert_eq!(tokens.to_string(), "0.0f64");
}

#[test]
fn test_to_tokens_with_negative_f64() {
    use quote::TokenStream;
    use quote::Literal;

    struct MyStruct(f64);

    let value = MyStruct(-2.71);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    assert_eq!(tokens.to_string(), "-2.71f64");
}

#[test]
fn test_to_tokens_with_large_f64() {
    use quote::TokenStream;
    use quote::Literal;

    struct MyStruct(f64);

    let value = MyStruct(1e10);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    assert_eq!(tokens.to_string(), "10000000000.0f64");
}

#[test]
#[should_panic]
fn test_to_tokens_with_nan() {
    use quote::TokenStream;
    use quote::Literal;

    struct MyStruct(f64);

    let value = MyStruct(f64::NAN);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    assert_eq!(tokens.to_string(), "NaN");
}

