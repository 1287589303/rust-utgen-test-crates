// Answer 0

#[test]
fn test_fmt_with_single_group() {
    let group = Group {
        delimiter: Delimiter::Parenthesis,
        stream: TokenStream {
            inner: RcVec::new(vec![TokenTree::Ident(Ident {
                sym: Box::from("test_ident"),
                span: Span::call_site(),
                raw: false,
            })]),
        },
        span: Span::call_site(),
    };

    let token_stream = TokenStream {
        inner: RcVec::new(vec![TokenTree::Group(group)]),
    };

    let _ = format!("{}", token_stream);
}

#[test]
fn test_fmt_with_group_and_joint_spacing() {
    let group = Group {
        delimiter: Delimiter::Bracket,
        stream: TokenStream {
            inner: RcVec::new(vec![TokenTree::Punct(Punct::new(',', Spacing::Joint))]),
        },
        span: Span::call_site(),
    };

    let token_stream = TokenStream {
        inner: RcVec::new(vec![TokenTree::Group(group)]),
    };

    let _ = format!("{}", token_stream);
}

#[test]
fn test_fmt_with_group_and_alone_spacing() {
    let group = Group {
        delimiter: Delimiter::Brace,
        stream: TokenStream {
            inner: RcVec::new(vec![TokenTree::Punct(Punct::new('+', Spacing::Alone))]),
        },
        span: Span::call_site(),
    };

    let token_stream = TokenStream {
        inner: RcVec::new(vec![TokenTree::Group(group)]),
    };

    let _ = format!("{}", token_stream);
}

