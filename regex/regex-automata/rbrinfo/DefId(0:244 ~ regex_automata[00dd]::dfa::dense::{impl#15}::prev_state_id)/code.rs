fn prev_state_id(&self, id: StateID) -> StateID {
        self.to_state_id(self.to_index(id).checked_sub(1).unwrap())
    }