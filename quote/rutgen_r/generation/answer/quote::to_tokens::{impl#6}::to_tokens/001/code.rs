// Answer 0

#[test]
fn test_to_tokens_empty_string() {
    struct TestStruct<'a>(&'a str);
    
    let input = TestStruct("");
    let mut tokens = TokenStream::new();
    input.to_tokens(&mut tokens);
    
    let expected: TokenStream = quote! { "" }.into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_non_empty_string() {
    struct TestStruct<'a>(&'a str);
    
    let input = TestStruct("Hello, World!");
    let mut tokens = TokenStream::new();
    input.to_tokens(&mut tokens);
    
    let expected: TokenStream = quote! { "Hello, World!" }.into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_special_characters() {
    struct TestStruct<'a>(&'a str);
    
    let input = TestStruct("Line1\nLine2\tTabbed");
    let mut tokens = TokenStream::new();
    input.to_tokens(&mut tokens);
    
    let expected: TokenStream = quote! { "Line1\nLine2\tTabbed" }.into();
    assert_eq!(tokens.to_string(), expected.to_string());
}

