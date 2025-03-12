fn c_exactly(&self, hir: &Hir, n: u32) -> Result<ThompsonRef, Error> {
        self.c_concat((0..n).map(|_| self.c(hir)))
    }