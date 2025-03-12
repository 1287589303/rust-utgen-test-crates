fn next_state(&self, current: StateID, input: u8) -> StateID {
        let input = self.tt.classes.get(input);
        self.tt.state(current).next(input)
    }