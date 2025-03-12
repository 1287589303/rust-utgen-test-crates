// Answer 0

#[test]
fn test_extend_empty() {
    let mut token_stream = TokenStream { inner: RcVec { inner: Rc::new(Vec::new()) } };
    let tokens: Vec<TokenTree> = Vec::new();
    token_stream.extend(tokens.into_iter());
}

#[test]
fn test_extend_single_ident() {
    let mut token_stream = TokenStream { inner: RcVec { inner: Rc::new(Vec::new()) } };
    let tokens: Vec<TokenTree> = vec![TokenTree::Ident(Ident)];
    token_stream.extend(tokens.into_iter());
}

#[test]
fn test_extend_multiple_types() {
    let mut token_stream = TokenStream { inner: RcVec { inner: Rc::new(Vec::new()) } };
    let tokens: Vec<TokenTree> = vec![
        TokenTree::Ident(Ident),
        TokenTree::Punct(Punct),
        TokenTree::Literal(Literal),
        TokenTree::Group(Group),
    ];
    token_stream.extend(tokens.into_iter());
}

#[test]
fn test_extend_single_punct() {
    let mut token_stream = TokenStream { inner: RcVec { inner: Rc::new(Vec::new()) } };
    let tokens: Vec<TokenTree> = vec![TokenTree::Punct(Punct)];
    token_stream.extend(tokens.into_iter());
}

#[test]
fn test_extend_empty_tokentree_iterator() {
    let mut token_stream = TokenStream { inner: RcVec { inner: Rc::new(Vec::new()) } };
    let tokens: Vec<TokenTree> = vec![];
    token_stream.extend(tokens.into_iter());
}

#[test]
fn test_extend_multiple_literls() {
    let mut token_stream = TokenStream { inner: RcVec { inner: Rc::new(Vec::new()) } };
    let tokens: Vec<TokenTree> = vec![
        TokenTree::Literal(Literal),
        TokenTree::Literal(Literal),
        TokenTree::Literal(Literal),
    ];
    token_stream.extend(tokens.into_iter());
}

