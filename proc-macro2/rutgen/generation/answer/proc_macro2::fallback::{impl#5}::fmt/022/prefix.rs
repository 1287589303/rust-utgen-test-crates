// Answer 0

#[test]
fn test_display_tokenstream_with_ident_element_first() {
    let ident = Ident {
        sym: Box::from("example".into()),
        span: Span::call_site(),
        raw: false,
    };
    let ident_token = TokenTree::Ident(ident);
    
    let punct_joint = Punct::new('+', Spacing::Joint);
    let punct_alone = Punct::new(';', Spacing::Alone);
    
    let inner_vec = RcVec {
        inner: Rc::new(vec![punct_alone.clone(), ident_token.clone()]),
    };
    
    let token_stream = TokenStream { inner: inner_vec };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", token_stream);
}

#[test]
fn test_display_tokenstream_with_ident_element_second() {
    let ident = Ident {
        sym: Box::from("test".into()),
        span: Span::call_site(),
        raw: false,
    };
    let ident_token = TokenTree::Ident(ident);
    
    let group = Group {
        delimiter: Delimiter::parenthesis(),
        stream: TokenStream {
            inner: RcVec {
                inner: Rc::new(vec![]),
            },
        },
        span: Span::call_site(),
    };
    let group_token = TokenTree::Group(group);
    
    let inner_vec = RcVec {
        inner: Rc::new(vec![group_token, ident_token]),
    };
    
    let token_stream = TokenStream { inner: inner_vec };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", token_stream);
}

