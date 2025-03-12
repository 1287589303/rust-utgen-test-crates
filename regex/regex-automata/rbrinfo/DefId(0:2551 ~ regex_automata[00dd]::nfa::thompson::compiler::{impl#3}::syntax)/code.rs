pub fn syntax(
        &mut self,
        config: crate::util::syntax::Config,
    ) -> &mut Compiler {
        config.apply(&mut self.parser);
        self
    }