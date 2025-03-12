// Answer 0

#[derive(Default)]
struct TokenStream {
    data: Vec<String>,
}

trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
}

struct Example;

impl ToTokens for Example {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.data.push("Example Token".to_string());
    }
}

#[test]
fn test_to_tokens() {
    let example = Example::default();
    let mut tokens = TokenStream::default();
    
    example.to_tokens(&mut tokens);
    
    assert_eq!(tokens.data.len(), 1);
    assert_eq!(tokens.data[0], "Example Token");
}

#[test]
fn test_to_tokens_no_tokens() {
    let example = Example::default();
    let mut tokens = TokenStream::default();
    
    assert!(tokens.data.is_empty());
    
    example.to_tokens(&mut tokens);
    
    assert_eq!(tokens.data.len(), 1);
    assert_eq!(tokens.data[0], "Example Token");
}

