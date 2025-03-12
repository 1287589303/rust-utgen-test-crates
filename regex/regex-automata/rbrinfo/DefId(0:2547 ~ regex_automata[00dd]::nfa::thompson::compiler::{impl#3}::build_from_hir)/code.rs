pub fn build_from_hir(&self, expr: &Hir) -> Result<NFA, BuildError> {
        self.build_many_from_hir(&[expr])
    }