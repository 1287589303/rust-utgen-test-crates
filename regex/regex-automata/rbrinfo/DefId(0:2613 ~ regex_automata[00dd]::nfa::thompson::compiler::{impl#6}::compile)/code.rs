fn compile(
        &mut self,
        node: Vec<Transition>,
    ) -> Result<StateID, BuildError> {
        let hash = self.state.compiled.hash(&node);
        if let Some(id) = self.state.compiled.get(&node, hash) {
            return Ok(id);
        }
        let id = self.builder.add_sparse(node.clone())?;
        self.state.compiled.set(node, hash, id);
        Ok(id)
    }