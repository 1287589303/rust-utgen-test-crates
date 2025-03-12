pub fn assertion(e: Assertion) -> Ast {
        Ast::Assertion(Box::new(e))
    }