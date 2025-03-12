fn visit_post(&mut self, ast: &Ast) -> fmt::Result {
        match *ast {
            Ast::Empty(_) => Ok(()),
            Ast::Flags(ref x) => self.fmt_set_flags(x),
            Ast::Literal(ref x) => self.fmt_literal(x),
            Ast::Dot(_) => self.wtr.write_str("."),
            Ast::Assertion(ref x) => self.fmt_assertion(x),
            Ast::ClassPerl(ref x) => self.fmt_class_perl(x),
            Ast::ClassUnicode(ref x) => self.fmt_class_unicode(x),
            Ast::ClassBracketed(ref x) => self.fmt_class_bracketed_post(x),
            Ast::Repetition(ref x) => self.fmt_repetition(x),
            Ast::Group(ref x) => self.fmt_group_post(x),
            Ast::Alternation(_) => Ok(()),
            Ast::Concat(_) => Ok(()),
        }
    }