pub(crate) fn remap(&mut self, map: impl Fn(StateID) -> StateID) {
        // We could loop over each state ID and call 'remap_state' here, but
        // this is more direct: just map every transition directly. This
        // technically might do a little extra work since the alphabet length
        // is likely less than the stride, but if that is indeed an issue we
        // should benchmark it and fix it.
        for sid in self.tt.table_mut().iter_mut() {
            *sid = map(*sid);
        }
        for sid in self.st.table_mut().iter_mut() {
            *sid = map(*sid);
        }
    }