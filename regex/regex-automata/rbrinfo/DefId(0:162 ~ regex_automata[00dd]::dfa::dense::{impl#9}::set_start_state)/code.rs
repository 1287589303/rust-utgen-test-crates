pub(crate) fn set_start_state(
        &mut self,
        anchored: Anchored,
        start: Start,
        id: StateID,
    ) {
        assert!(self.tt.is_valid(id), "invalid start state");
        self.st.set_start(anchored, start, id);
    }