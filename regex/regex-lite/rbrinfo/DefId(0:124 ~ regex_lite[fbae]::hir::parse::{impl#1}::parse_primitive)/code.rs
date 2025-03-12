fn parse_primitive(&self) -> Result<Hir, Error> {
        let ch = self.char();
        self.bump();
        match ch {
            '\\' => self.parse_escape(),
            '.' => Ok(self.hir_dot()),
            '^' => Ok(self.hir_anchor_start()),
            '$' => Ok(self.hir_anchor_end()),
            ch => Ok(self.hir_char(ch)),
        }
    }