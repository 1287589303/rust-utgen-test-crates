fn next(&mut self) -> Option<TokenTree> {
        let token = match self {
            TokenTreeIter::Compiler(iter) => iter.next()?,
            TokenTreeIter::Fallback(iter) => return iter.next(),
        };
        Some(match token {
            proc_macro::TokenTree::Group(tt) => {
                TokenTree::Group(crate::Group::_new(Group::Compiler(tt)))
            }
            proc_macro::TokenTree::Punct(tt) => {
                let spacing = match tt.spacing() {
                    proc_macro::Spacing::Joint => Spacing::Joint,
                    proc_macro::Spacing::Alone => Spacing::Alone,
                };
                let mut o = Punct::new(tt.as_char(), spacing);
                o.set_span(crate::Span::_new(Span::Compiler(tt.span())));
                TokenTree::Punct(o)
            }
            proc_macro::TokenTree::Ident(s) => {
                TokenTree::Ident(crate::Ident::_new(Ident::Compiler(s)))
            }
            proc_macro::TokenTree::Literal(l) => {
                TokenTree::Literal(crate::Literal::_new(Literal::Compiler(l)))
            }
        })
    }