pub(crate) fn state(&self, id: StateID) -> &State {
        &self.states[id.as_usize()]
    }