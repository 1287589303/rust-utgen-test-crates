// Answer 0

#[test]
fn test_extend_empty_streams() {
    let mut stream = TokenStream {
        inner: RcVec {
            inner: Rc::new(Vec::new()),
        },
    };
    let empty_streams: Vec<TokenStream> = Vec::new();
    stream.extend(empty_streams);
}

#[test]
fn test_extend_single_element_stream() {
    let mut stream = TokenStream {
        inner: RcVec {
            inner: Rc::new(Vec::new()),
        },
    };
    let single_stream = TokenStream {
        inner: RcVec {
            inner: Rc::new(vec![TokenTree::Ident(Ident::new("x", Span::call_site()))]),
        },
    };
    let single_element_streams = vec![single_stream];
    stream.extend(single_element_streams);
}

#[test]
fn test_extend_multiple_element_streams() {
    let mut stream = TokenStream {
        inner: RcVec {
            inner: Rc::new(Vec::new()),
        },
    };
    let multiple_streams = vec![
        TokenStream {
            inner: RcVec {
                inner: Rc::new(vec![TokenTree::Ident(Ident::new("a", Span::call_site()))]),
            },
        },
        TokenStream {
            inner: RcVec {
                inner: Rc::new(vec![TokenTree::Punct(Punct::new('+', Spacing::Alone))]),
            },
        },
        TokenStream {
            inner: RcVec {
                inner: Rc::new(vec![TokenTree::Literal(Literal::new("42", Span::call_site()))]),
            },
        },
    ];
    stream.extend(multiple_streams);
}

#[test]
fn test_extend_with_empty_and_non_empty_streams() {
    let mut stream = TokenStream {
        inner: RcVec {
            inner: Rc::new(Vec::new()),
        },
    };
    let empty_streams: Vec<TokenStream> = Vec::new();
    let non_empty_stream = TokenStream {
        inner: RcVec {
            inner: Rc::new(vec![TokenTree::Ident(Ident::new("y", Span::call_site()))]),
        },
    };
    let mixed_streams = vec![empty_streams, non_empty_stream];
    stream.extend(mixed_streams.into_iter().flatten());
}

