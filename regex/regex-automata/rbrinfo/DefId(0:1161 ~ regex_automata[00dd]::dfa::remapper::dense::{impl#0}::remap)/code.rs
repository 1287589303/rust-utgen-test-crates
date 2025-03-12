fn remap(&mut self, map: impl Fn(StateID) -> StateID) {
            OwnedDFA::remap(self, map)
        }