fn reset(&mut self, re: &PikeVM) {
        let nfa = re.get_nfa();
        self.slots_per_state = nfa.group_info().slot_len();
        // This is always correct, but may be reduced for a particular search
        // if a 'Captures' has fewer slots, e.g., none at all or only slots
        // for tracking the overall match instead of all slots for every
        // group.
        self.slots_for_captures = core::cmp::max(
            self.slots_per_state,
            nfa.pattern_len().checked_mul(2).unwrap(),
        );
        let len = nfa
            .states()
            .len()
            .checked_mul(self.slots_per_state)
            // Add space to account for scratch space used during a search.
            .and_then(|x| x.checked_add(self.slots_for_captures))
            // It seems like this could actually panic on legitimate inputs on
            // 32-bit targets, and very likely to panic on 16-bit. Should we
            // somehow convert this to an error? What about something similar
            // for the lazy DFA cache? If you're tripping this assert, please
            // file a bug.
            .expect("slot table length doesn't overflow");
        // This happens about as often as a regex is compiled, so it probably
        // should be at debug level, but I found it quite distracting and not
        // particularly useful.
        trace!(
            "resizing PikeVM active states table to {} entries \
             (slots_per_state={})",
            len,
            self.slots_per_state,
        );
        self.table.resize(len, None);
    }