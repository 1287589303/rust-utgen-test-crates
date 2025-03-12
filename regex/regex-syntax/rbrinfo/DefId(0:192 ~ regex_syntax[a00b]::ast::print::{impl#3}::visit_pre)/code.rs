fn visit_pre(&mut self, ast: &Ast) -> fmt::Result {
        match *ast {
            Ast::Group(ref x) => self.fmt_group_pre(x),
            Ast::ClassBracketed(ref x) => self.fmt_class_bracketed_pre(x),
            _ => Ok(()),
        }
    }