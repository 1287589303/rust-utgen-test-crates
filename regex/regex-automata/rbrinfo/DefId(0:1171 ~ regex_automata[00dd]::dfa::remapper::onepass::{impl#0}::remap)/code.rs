fn remap(&mut self, map: impl Fn(StateID) -> StateID) {
            DFA::remap(self, map)
        }