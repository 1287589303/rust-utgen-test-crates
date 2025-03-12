fn into_ast(self) -> Ast {
        match self {
            Primitive::Literal(lit) => Ast::literal(lit),
            Primitive::Assertion(assert) => Ast::assertion(assert),
            Primitive::Dot(span) => Ast::dot(span),
            Primitive::Perl(cls) => Ast::class_perl(cls),
            Primitive::Unicode(cls) => Ast::class_unicode(cls),
        }
    }