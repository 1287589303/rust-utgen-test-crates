// Answer 0

#[test]
fn test_rcvec_is_empty_true() {
    let rc_vec: RcVec<TokenTree> = RcVec {
        inner: Rc::new(Vec::new()),
    };
    assert!(rc_vec.is_empty());
}

#[test]
fn test_rcvec_is_empty_false() {
    let rc_vec: RcVec<TokenTree> = RcVec {
        inner: Rc::new(vec![TokenTree::Ident(Ident::new("test", Span::call_site()))]),
    };
    assert!(!rc_vec.is_empty());
}

#[test]
fn test_tokenstream_is_empty_true() {
    let token_stream = TokenStream::new();
    assert!(token_stream.is_empty());
}

#[test]
fn test_tokenstream_is_empty_false() {
    let rc_vec: RcVec<TokenTree> = RcVec {
        inner: Rc::new(vec![TokenTree::Ident(Ident::new("test", Span::call_site()))]),
    };
    let token_stream = TokenStream { inner: rc_vec };
    assert!(!token_stream.is_empty());
}

