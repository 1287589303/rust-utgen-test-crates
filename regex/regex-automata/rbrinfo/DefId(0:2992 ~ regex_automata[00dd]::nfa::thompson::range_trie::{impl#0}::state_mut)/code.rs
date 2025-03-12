fn state_mut(&mut self, id: StateID) -> &mut State {
        &mut self.states[id]
    }