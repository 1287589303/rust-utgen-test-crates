fn state(&self, id: StateID) -> State<'_> {
        assert!(self.is_valid(id));

        let i = id.as_usize();
        State {
            id,
            stride2: self.stride2,
            transitions: &self.table()[i..i + self.alphabet_len()],
        }
    }