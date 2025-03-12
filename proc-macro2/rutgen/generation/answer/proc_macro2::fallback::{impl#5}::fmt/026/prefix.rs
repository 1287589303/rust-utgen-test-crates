// Answer 0

#[test]
fn test_fmt_empty_token_stream() {
    let empty_stream = TokenStream {
        inner: RcVec::new(),
    };
    let mut output = String::new();
    empty_stream.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_empty_rcvec() {
    let empty_rcvec: RcVec<TokenTree> = RcVec {
        inner: Rc::new(Vec::new()),
    };
    let empty_stream = TokenStream {
        inner: empty_rcvec,
    };
    let mut output = String::new();
    empty_stream.fmt(&mut output).unwrap();
}

