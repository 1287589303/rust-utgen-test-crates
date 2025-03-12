fn push_token_from_proc_macro(mut vec: RcVecMut<TokenTree>, token: TokenTree) {
    // https://github.com/dtolnay/proc-macro2/issues/235
    match token {
        TokenTree::Literal(crate::Literal {
            #[cfg(wrap_proc_macro)]
                inner: crate::imp::Literal::Fallback(literal),
            #[cfg(not(wrap_proc_macro))]
                inner: literal,
            ..
        }) if literal.repr.starts_with('-') => {
            push_negative_literal(vec, literal);
        }
        _ => vec.push(token),
    }

    #[cold]
    fn push_negative_literal(mut vec: RcVecMut<TokenTree>, mut literal: Literal) {
        literal.repr.remove(0);
        let mut punct = crate::Punct::new('-', Spacing::Alone);
        punct.set_span(crate::Span::_new_fallback(literal.span));
        vec.push(TokenTree::Punct(punct));
        vec.push(TokenTree::Literal(crate::Literal::_new_fallback(literal)));
    }
}