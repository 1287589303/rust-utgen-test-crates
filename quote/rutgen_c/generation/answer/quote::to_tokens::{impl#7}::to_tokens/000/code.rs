// Answer 0

#[test]
fn test_to_tokens_bool() {
    use proc_macro2::{TokenStream, Ident};
    use super::ToTokens;

    // Create a mutable TokenStream to test against
    let mut tokens = TokenStream::new();

    // Test for the true value
    let value_true: bool = true;
    value_true.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "true");

    // Clear tokens for the next test
    tokens = TokenStream::new();

    // Test for the false value
    let value_false: bool = false;
    value_false.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "false");
}

