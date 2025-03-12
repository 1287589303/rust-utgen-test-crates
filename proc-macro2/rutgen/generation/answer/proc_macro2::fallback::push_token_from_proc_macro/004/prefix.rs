// Answer 0

#[test]
fn test_push_token_from_proc_macro_with_positive_literal() {
    let mut vec = RcVecMut {
        inner: &mut Vec::new(),
    };
    let token = TokenTree::Literal(Literal {
        repr: String::from("5"),
        span: Span::default(),
    });
    push_token_from_proc_macro(vec, token);
}

#[test]
fn test_push_token_from_proc_macro_with_string_literal() {
    let mut vec = RcVecMut {
        inner: &mut Vec::new(),
    };
    let token = TokenTree::Literal(Literal {
        repr: String::from("abc"),
        span: Span::default(),
    });
    push_token_from_proc_macro(vec, token);
}

#[test]
fn test_push_token_from_proc_macro_with_large_number() {
    let mut vec = RcVecMut {
        inner: &mut Vec::new(),
    };
    let token = TokenTree::Literal(Literal {
        repr: String::from("1000000"),
        span: Span::default(),
    });
    push_token_from_proc_macro(vec, token);
}

