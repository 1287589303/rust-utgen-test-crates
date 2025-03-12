fn next_state_id(&self, id: StateID) -> StateID {
        self.to_state_id(self.to_index(id).checked_add(1).unwrap())
    }