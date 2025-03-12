fn universal_start_state(&self, mode: Anchored) -> Option<StateID> {
        match mode {
            Anchored::No => self.st.universal_start_unanchored,
            Anchored::Yes => self.st.universal_start_anchored,
            Anchored::Pattern(_) => None,
        }
    }