// Answer 0

#[derive(Clone)]
struct TokenStream {
    data: Vec<String>,
}

impl TokenStream {
    fn new() -> Self {
        TokenStream { data: Vec::new() }
    }

    fn append(&mut self, token: String) {
        self.data.push(token);
    }
}

trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
}

#[derive(Clone)]
struct MyStruct {
    value: String,
}

impl ToTokens for MyStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.value.clone());
    }
}

#[test]
fn test_to_tokens_single_value() {
    let my_struct = MyStruct {
        value: "token1".to_string(),
    };
    let mut tokens = TokenStream::new();
    my_struct.to_tokens(&mut tokens);
    
    assert_eq!(tokens.data.len(), 1);
    assert_eq!(tokens.data[0], "token1");
}

#[test]
fn test_to_tokens_multiple_values() {
    let my_struct1 = MyStruct {
        value: "token1".to_string(),
    };
    let my_struct2 = MyStruct {
        value: "token2".to_string(),
    };
    
    let mut tokens = TokenStream::new();
    my_struct1.to_tokens(&mut tokens);
    my_struct2.to_tokens(&mut tokens);
    
    assert_eq!(tokens.data.len(), 2);
    assert_eq!(tokens.data[0], "token1");
    assert_eq!(tokens.data[1], "token2");
}

