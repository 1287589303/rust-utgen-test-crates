fn new(
        builder: &'a mut Builder,
        state: &'a mut Utf8State,
    ) -> Result<Utf8Compiler<'a>, BuildError> {
        let target = builder.add_empty()?;
        state.clear();
        let mut utf8c = Utf8Compiler { builder, state, target };
        utf8c.add_empty();
        Ok(utf8c)
    }