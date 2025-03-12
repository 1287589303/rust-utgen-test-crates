// Answer 0

#[test]
fn test_append_all_empty_iterator() {
    struct DummyToTokens;

    impl ToTokens for DummyToTokens {
        fn to_tokens(&self, _tokens: &mut Tokens) {}
    }

    let mut tokens = Tokens::new();
    let iter: Vec<DummyToTokens> = Vec::new();
    
    tokens.append_all(iter);
    
    // Assert the tokens haven't changed (remains empty)
    assert!(tokens.is_empty());
}

#[test]
fn test_append_all_single_token() {
    struct DummyToTokens;

    impl ToTokens for DummyToTokens {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.push("single_token".to_string());
        }
    }

    let mut tokens = Tokens::new();
    let iter = vec![DummyToTokens]; // Only one token
    
    tokens.append_all(iter);
    
    // Assert that the token was added
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], "single_token");
}

#[test]
fn test_append_all_multiple_tokens() {
    struct DummyToTokens;

    impl ToTokens for DummyToTokens {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.push("token".to_string());
        }
    }

    let mut tokens = Tokens::new();
    let iter = vec![DummyToTokens, DummyToTokens]; // Two tokens
    
    tokens.append_all(iter);
    
    // Assert that the tokens were added
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], "token");
    assert_eq!(tokens[1], "token");
}

