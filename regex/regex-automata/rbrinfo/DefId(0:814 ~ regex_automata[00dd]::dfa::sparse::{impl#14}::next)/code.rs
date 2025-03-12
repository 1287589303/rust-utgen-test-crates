fn next(&mut self) -> Option<(StateID, Anchored, Start)> {
        let i = self.i;
        if i >= self.st.len() {
            return None;
        }
        self.i += 1;

        // This unwrap is okay since the stride of any DFA must always match
        // the number of start state types.
        let start_type = Start::from_usize(i % self.st.stride).unwrap();
        let anchored = if i < self.st.stride {
            Anchored::No
        } else if i < (2 * self.st.stride) {
            Anchored::Yes
        } else {
            let pid = (i - (2 * self.st.stride)) / self.st.stride;
            Anchored::Pattern(PatternID::new(pid).unwrap())
        };
        let start = i * StateID::SIZE;
        let end = start + StateID::SIZE;
        let bytes = self.st.table()[start..end].try_into().unwrap();
        // This is OK since we're allowed to assume that any IDs in this start
        // table are correct and valid for this DFA.
        let id = StateID::from_ne_bytes_unchecked(bytes);
        Some((id, anchored, start_type))
    }