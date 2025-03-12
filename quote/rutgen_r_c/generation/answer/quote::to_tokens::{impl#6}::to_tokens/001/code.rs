// Answer 0

#[test]
fn test_to_tokens_string() {
    struct TestString<'a>(&'a str);
    
    impl ToTokens for TestString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }
    
    let test_string = TestString("Hello, world!");
    let mut tokens = TokenStream::new();
    test_string.to_tokens(&mut tokens);
    
    let expected_tokens = TokenStream::from(Literal::string("Hello, world!"));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_empty_string() {
    struct TestString<'a>(&'a str);
    
    impl ToTokens for TestString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }
    
    let test_string = TestString("");
    let mut tokens = TokenStream::new();
    test_string.to_tokens(&mut tokens);
    
    let expected_tokens = TokenStream::from(Literal::string(""));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_special_characters() {
    struct TestString<'a>(&'a str);
    
    impl ToTokens for TestString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }
    
    let test_string = TestString("Hello, \n world!");
    let mut tokens = TokenStream::new();
    test_string.to_tokens(&mut tokens);
    
    let expected_tokens = TokenStream::from(Literal::string("Hello, \n world!"));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

