fn c_exactly(
        &self,
        expr: &Hir,
        n: u32,
    ) -> Result<ThompsonRef, BuildError> {
        let it = (0..n).map(|_| self.c(expr));
        self.c_concat(it)
    }