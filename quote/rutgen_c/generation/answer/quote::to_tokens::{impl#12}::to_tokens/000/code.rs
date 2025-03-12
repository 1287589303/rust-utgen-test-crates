// Answer 0

#[test]
fn test_to_tokens_bool_true() {
    use proc_macro2::{TokenStream, Literal};
    struct TestValue(bool);

    let value = TestValue(true);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    let expected = Literal::i128_suffixed(1);
    assert!(tokens.to_string().contains(&expected.to_string()));
}

#[test]
fn test_to_tokens_bool_false() {
    use proc_macro2::{TokenStream, Literal};
    struct TestValue(bool);

    let value = TestValue(false);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    let expected = Literal::i128_suffixed(0);
    assert!(tokens.to_string().contains(&expected.to_string()));
}

