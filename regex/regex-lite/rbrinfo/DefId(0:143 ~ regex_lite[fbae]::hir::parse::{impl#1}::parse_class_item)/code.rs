fn parse_class_item(&self) -> Result<Hir, Error> {
        let ch = self.char();
        self.bump();
        if ch == '\\' {
            self.parse_escape()
        } else {
            Ok(Hir::char(ch))
        }
    }