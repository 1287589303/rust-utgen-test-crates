// Answer 0

#[derive(Debug)]
struct TokenTree;

struct TokenAppender {
    tokens: Vec<TokenTree>,
}

impl TokenAppender {
    fn new() -> Self {
        TokenAppender { tokens: Vec::new() }
    }

    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {
        self.tokens.extend(std::iter::once(token.into()));
    }
}

#[test]
fn test_append_single_token() {
    let mut appender = TokenAppender::new();
    let token = TokenTree;
    appender.append(token);
    assert_eq!(appender.tokens.len(), 1);
}

#[test]
fn test_append_multiple_tokens() {
    let mut appender = TokenAppender::new();
    let token1 = TokenTree;
    let token2 = TokenTree;
    appender.append(token1);
    appender.append(token2);
    assert_eq!(appender.tokens.len(), 2);
}

#[test]
fn test_append_empty() {
    let mut appender = TokenAppender::new();
    assert_eq!(appender.tokens.len(), 0);
    appender.append(TokenTree);
    assert_eq!(appender.tokens.len(), 1);
}

