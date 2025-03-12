fn reset(&mut self, re: &PikeVM) {
        let nfa = re.nfa();
        // OK because NFA construction would have failed if this overflowed.
        self.slots_per_state = nfa.group_len().checked_mul(2).unwrap();
        // This is always correct, but may be reduced for a particular search
        // if fewer slots were given by the caller, e.g., none at all or only
        // slots for tracking the overall match instead of all slots for every
        // group.
        self.slots_for_captures = self.slots_per_state;
        let len = nfa
            .len()
            // We add 1 so that our last row is always empty. We use it as
            // "scratch" space for computing the epsilon closure off of the
            // starting state.
            .checked_add(1)
            .and_then(|x| x.checked_mul(self.slots_per_state))
            // It seems like this could actually panic on legitimate inputs
            // on 32-bit targets. Should we somehow convert this to an error?
            // What about something similar for the lazy DFA cache? If you're
            // tripping this assert, please file a bug.
            .expect("slot table length doesn't overflow");
        self.table.resize(len, None);
    }