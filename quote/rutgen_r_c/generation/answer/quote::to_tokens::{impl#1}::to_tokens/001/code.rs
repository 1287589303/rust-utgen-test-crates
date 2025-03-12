// Answer 0

#[test]
fn test_to_tokens_true() {
    struct BoolWrapper(bool);
    
    let value = BoolWrapper(true);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected = "true";
    let actual: String = tokens.to_string();
    assert_eq!(actual, expected);
}

#[test]
fn test_to_tokens_false() {
    struct BoolWrapper(bool);
    
    let value = BoolWrapper(false);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
    
    let expected = "false";
    let actual: String = tokens.to_string();
    assert_eq!(actual, expected);
}

