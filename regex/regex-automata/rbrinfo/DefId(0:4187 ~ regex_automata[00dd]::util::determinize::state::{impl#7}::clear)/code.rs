pub(crate) fn clear(self) -> StateBuilderEmpty {
        let mut builder = StateBuilderEmpty(self.repr);
        builder.clear();
        builder
    }