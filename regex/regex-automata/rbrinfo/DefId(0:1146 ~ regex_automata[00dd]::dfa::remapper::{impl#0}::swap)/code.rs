pub(super) fn swap(
        &mut self,
        r: &mut impl Remappable,
        id1: StateID,
        id2: StateID,
    ) {
        if id1 == id2 {
            return;
        }
        r.swap_states(id1, id2);
        self.map.swap(self.idxmap.to_index(id1), self.idxmap.to_index(id2));
    }