fn next(&mut self) -> Option<State<'a>> {
        if self.id >= self.trans.sparse().len() {
            return None;
        }
        let state = self.trans.state(StateID::new_unchecked(self.id));
        self.id = self.id + state.write_to_len();
        Some(state)
    }