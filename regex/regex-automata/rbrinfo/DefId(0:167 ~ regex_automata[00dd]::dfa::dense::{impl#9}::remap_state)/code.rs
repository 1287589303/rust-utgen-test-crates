pub(crate) fn remap_state(
        &mut self,
        id: StateID,
        map: impl Fn(StateID) -> StateID,
    ) {
        self.tt.remap(id, map);
    }