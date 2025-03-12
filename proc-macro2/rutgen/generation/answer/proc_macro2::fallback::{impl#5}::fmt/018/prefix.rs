// Answer 0

#[test]
fn test_fmt_single_literal() {
    use std::fmt::Write;

    let literal = Literal {
        repr: "test".to_string(),
        span: Span::call_site(),
    };

    let token_tree = TokenTree::Literal(literal);
    let inner_vec = RcVec {
        inner: Rc::new(vec![token_tree]),
    };

    let token_stream = TokenStream { inner: inner_vec };
    
    let mut output = String::new();
    let _ = write!(output, "{}", token_stream);
}

#[test]
#[should_panic]
fn test_fmt_no_second_item() {
    use std::fmt::Write;

    let literal = Literal {
        repr: "example".to_string(),
        span: Span::call_site(),
    };

    let token_tree = TokenTree::Literal(literal);
    let inner_vec = RcVec {
        inner: Rc::new(vec![token_tree]),
    };

    let token_stream = TokenStream { inner: inner_vec };
    
    let mut output = String::new();
    let _ = write!(output, "{}", token_stream);
}

