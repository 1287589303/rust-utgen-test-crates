fn set_state_id(&mut self, sid: StateID) {
        *self = Transition::new(self.match_wins(), sid, self.epsilons());
    }