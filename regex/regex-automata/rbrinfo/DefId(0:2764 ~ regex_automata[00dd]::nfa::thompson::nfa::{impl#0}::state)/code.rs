pub fn state(&self, id: StateID) -> &State {
        &self.states()[id]
    }