pub(crate) fn apply_ast(&self, builder: &mut ast::parse::ParserBuilder) {
        builder
            .ignore_whitespace(self.ignore_whitespace)
            .nest_limit(self.nest_limit)
            .octal(self.octal);
    }