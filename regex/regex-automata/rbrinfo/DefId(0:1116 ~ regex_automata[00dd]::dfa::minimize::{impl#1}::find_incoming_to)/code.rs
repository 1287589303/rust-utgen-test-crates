fn find_incoming_to(
        &self,
        b: alphabet::Unit,
        set: &StateSet,
        incoming: &mut StateSet,
    ) {
        incoming.clear();
        set.iter(|id| {
            for &inid in
                &self.in_transitions[self.dfa.to_index(id)][b.as_usize()]
            {
                incoming.add(inid);
            }
        });
        incoming.canonicalize();
    }