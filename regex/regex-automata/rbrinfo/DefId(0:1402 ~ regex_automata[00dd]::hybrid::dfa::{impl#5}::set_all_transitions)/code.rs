fn set_all_transitions(&mut self, from: LazyStateID, to: LazyStateID) {
        for unit in self.dfa.classes.representatives(..) {
            self.set_transition(from, unit, to);
        }
    }