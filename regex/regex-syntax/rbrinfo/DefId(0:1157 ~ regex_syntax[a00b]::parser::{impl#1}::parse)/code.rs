pub fn parse(&mut self, pattern: &str) -> Result<hir::Hir, Error> {
        let ast = self.ast.parse(pattern)?;
        let hir = self.hir.translate(pattern, &ast)?;
        Ok(hir)
    }