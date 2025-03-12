pub(crate) fn new(
        config: Config,
        pattern: String,
        hir: &Hir,
    ) -> Result<NFA, Error> {
        Compiler::new(config, pattern).compile(hir)
    }