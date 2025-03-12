pub(crate) fn set_transition(
        &mut self,
        from: StateID,
        byte: alphabet::Unit,
        to: StateID,
    ) {
        self.tt.set(from, byte, to);
    }