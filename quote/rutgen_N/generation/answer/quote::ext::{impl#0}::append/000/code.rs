// Answer 0

#[derive(Debug)]
struct TokenTree;

struct TokenList(Vec<TokenTree>);

impl TokenList {
    fn new() -> Self {
        TokenList(Vec::new())
    }

    fn extend<I>(&mut self, iter: I)
    where
        I: Iterator<Item = TokenTree>,
    {
        self.0.extend(iter);
    }

    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {
        self.extend(std::iter::once(token.into()));
    }
}

#[test]
fn test_append_single_token() {
    let mut token_list = TokenList::new();
    let token = TokenTree; // A single token to append
    token_list.append(token);
    assert_eq!(token_list.0.len(), 1);
}

#[test]
fn test_append_multiple_tokens() {
    let mut token_list = TokenList::new();

    let token1 = TokenTree;
    let token2 = TokenTree;

    token_list.append(token1);
    token_list.append(token2);

    assert_eq!(token_list.0.len(), 2);
}

#[test]
fn test_append_empty() {
    let mut token_list = TokenList::new();
    token_list.append(TokenTree); // still appending one empty token
    assert_eq!(token_list.0.len(), 1);
}

