fn next(&mut self) -> Option<(StateID, Anchored, Start)> {
        let i = self.i;
        let table = self.st.table();
        if i >= table.len() {
            return None;
        }
        self.i += 1;

        // This unwrap is okay since the stride of the starting state table
        // must always match the number of start state types.
        let start_type = Start::from_usize(i % self.st.stride).unwrap();
        let anchored = if i < self.st.stride {
            Anchored::No
        } else if i < (2 * self.st.stride) {
            Anchored::Yes
        } else {
            let pid = (i - (2 * self.st.stride)) / self.st.stride;
            Anchored::Pattern(PatternID::new(pid).unwrap())
        };
        Some((table[i], anchored, start_type))
    }