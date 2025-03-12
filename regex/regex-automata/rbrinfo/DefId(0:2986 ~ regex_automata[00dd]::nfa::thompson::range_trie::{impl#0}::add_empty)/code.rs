pub fn add_empty(&mut self) -> StateID {
        let id = match StateID::try_from(self.states.len()) {
            Ok(id) => id,
            Err(_) => {
                // This generally should not happen since a range trie is
                // only ever used to compile a single sequence of Unicode
                // scalar values. If we ever got to this point, we would, at
                // *minimum*, be using 96GB in just the range trie alone.
                panic!("too many sequences added to range trie");
            }
        };
        // If we have some free states available, then use them to avoid
        // more allocations.
        if let Some(mut state) = self.free.pop() {
            state.clear();
            self.states.push(state);
        } else {
            self.states.push(State { transitions: vec![] });
        }
        id
    }