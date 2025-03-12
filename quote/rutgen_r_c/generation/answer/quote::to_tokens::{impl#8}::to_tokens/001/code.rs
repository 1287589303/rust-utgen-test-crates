// Answer 0

#[test]
fn test_to_tokens_true() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct BooleanWrapper(bool);

    // Create a TokenStream to hold the tokens
    let mut tokens = TokenStream::new();
    let boolean = BooleanWrapper(true);

    // Call the to_tokens method
    boolean.to_tokens(&mut tokens);

    // Check if the tokens represent the literal "1_i8"
    let expected = Literal::i8_suffixed(1);
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_false() {
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct BooleanWrapper(bool);

    // Create a TokenStream to hold the tokens
    let mut tokens = TokenStream::new();
    let boolean = BooleanWrapper(false);

    // Call the to_tokens method
    boolean.to_tokens(&mut tokens);

    // Check if the tokens represent the literal "0_i8"
    let expected = Literal::i8_suffixed(0);
    assert_eq!(tokens.to_string(), expected.to_string());
}

