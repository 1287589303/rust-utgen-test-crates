// Answer 0

#[test]
fn test_fmt_with_multiple_token_tree_elements() {
    let group = Group {
        delimiter: Delimiter::Parenthesis,
        stream: TokenStream {
            inner: RcVec::new(vec![TokenTree::Ident(Ident {
                sym: Box::from("example"),
                span: Span::call_site(),
                raw: false,
            })]),
        },
        span: Span::call_site(),
    };

    let punct_joint = Punct::new('+', Spacing::Joint);
    let punct_alone = Punct::new(',', Spacing::Alone);
    let literal = Literal { repr: String::from("42"), span: Span::call_site() };
    
    let token_stream = TokenStream {
        inner: RcVec::new(vec![
            TokenTree::Group(group),
            TokenTree::Punct(punct_joint),
            TokenTree::Ident(Ident {
                sym: Box::from("test"),
                span: Span::call_site(),
                raw: true,
            }),
            TokenTree::Punct(punct_alone),
            TokenTree::Literal(literal),
        ]),
    };

    let _ = write!(&mut String::new(), "{}", token_stream);
}

#[test]
fn test_fmt_with_group_and_joint_punctuation() {
    let group = Group {
        delimiter: Delimiter::Bracket,
        stream: TokenStream {
            inner: RcVec::new(vec![TokenTree::Ident(Ident {
                sym: Box::from("grouped"),
                span: Span::call_site(),
                raw: false,
            })]),
        },
        span: Span::call_site(),
    };

    let joint_punct = Punct::new('*', Spacing::Joint);
    let lone_punct = Punct::new('%', Spacing::Alone);
    
    let token_stream = TokenStream {
        inner: RcVec::new(vec![
            TokenTree::Group(group),
            TokenTree::Punct(joint_punct),
            TokenTree::Punct(lone_punct),
        ]),
    };

    let _ = write!(&mut String::new(), "{}", token_stream);
}

