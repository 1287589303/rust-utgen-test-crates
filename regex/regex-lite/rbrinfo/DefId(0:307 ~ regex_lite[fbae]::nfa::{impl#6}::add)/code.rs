fn add(&self, state: State) -> Result<StateID, Error> {
        let id = u32::try_from(self.nfa.borrow().states.len())
            .map_err(|_| Error::new("exhausted state IDs, too many states"))?;
        self.nfa.borrow_mut().memory_extra += state.memory_usage();
        self.nfa.borrow_mut().states.push(state);
        self.check_size_limit()?;
        Ok(id)
    }