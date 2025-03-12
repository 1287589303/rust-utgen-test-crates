pub fn literal(e: Literal) -> Ast {
        Ast::Literal(Box::new(e))
    }