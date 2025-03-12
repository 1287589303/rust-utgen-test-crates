// Answer 0

#[test]
fn test_push_empty_token_tree() {
    let mut builder = TokenStreamBuilder::new();
    let token = TokenTree::Ident(Ident::new("empty", Span::call_site()));
    builder.push_token_from_parser(token);
}

#[test]
fn test_push_ident_token_tree() {
    let mut builder = TokenStreamBuilder::new();
    let token = TokenTree::Ident(Ident::new("identifier", Span::call_site()));
    builder.push_token_from_parser(token);
}

#[test]
fn test_push_punct_token_tree() {
    let mut builder = TokenStreamBuilder::new();
    let token = TokenTree::Punct(Punct::new('+', Spacing::Alone));
    builder.push_token_from_parser(token);
}

#[test]
fn test_push_literal_token_tree() {
    let mut builder = TokenStreamBuilder::new();
    let token = TokenTree::Literal(Literal::new("42", Span::call_site()));
    builder.push_token_from_parser(token);
}

#[test]
fn test_push_group_token_tree() {
    let mut builder = TokenStreamBuilder::new();
    let token = TokenTree::Group(Group::new(Delimiter::Parenthesis, RcVec::new()));
    builder.push_token_from_parser(token);
}

#[test]
fn test_push_large_capacity_token_tree() {
    let mut builder = TokenStreamBuilder::with_capacity(100);
    for i in 0..100 {
        let token = TokenTree::Ident(Ident::new(&format!("ident_{}", i), Span::call_site()));
        builder.push_token_from_parser(token);
    }
}

#[test]
fn test_push_boundary_capacity_token_tree() {
    let mut builder = TokenStreamBuilder::with_capacity(0);
    let token = TokenTree::Literal(Literal::new("0", Span::call_site()));
    builder.push_token_from_parser(token);
}

#[test]
fn test_push_exceeding_capacity_token_tree() {
    let mut builder = TokenStreamBuilder::with_capacity(1);
    let token1 = TokenTree::Ident(Ident::new("first", Span::call_site()));
    builder.push_token_from_parser(token1);
    let token2 = TokenTree::Punct(Punct::new('-', Spacing::Joint));
    builder.push_token_from_parser(token2);
}

