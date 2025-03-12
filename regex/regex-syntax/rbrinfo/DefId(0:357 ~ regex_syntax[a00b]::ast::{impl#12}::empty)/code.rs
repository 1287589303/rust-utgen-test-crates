pub fn empty(span: Span) -> Ast {
        Ast::Empty(Box::new(span))
    }