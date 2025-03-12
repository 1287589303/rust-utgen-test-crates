// Answer 0

#[test]
fn test_to_tokens_with_empty_cstring() {
    use proc_macro2::TokenStream;
    let cstr = CString::new("").unwrap();
    let mut tokens = TokenStream::new();
    cstr.to_tokens(&mut tokens);
    let expected_tokens: TokenStream = Literal::c_string(&cstr).into();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_with_special_characters() {
    use proc_macro2::TokenStream;
    let cstr = CString::new("Hello, world!").unwrap();
    let mut tokens = TokenStream::new();
    cstr.to_tokens(&mut tokens);
    let expected_tokens: TokenStream = Literal::c_string(&cstr).into();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_with_null_character() {
    use proc_macro2::TokenStream;
    let cstr = CString::new("Hello\0World").unwrap();
    let mut tokens = TokenStream::new();
    cstr.to_tokens(&mut tokens);
    let expected_tokens: TokenStream = Literal::c_string(&cstr).into();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

