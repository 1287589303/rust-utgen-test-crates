fn add_all_starts(
        &mut self,
        dfa_state_ids: &mut Vec<StateID>,
    ) -> Result<(), BuildError> {
        // These should be the first states added.
        assert!(dfa_state_ids.is_empty());
        // We only want to add (un)anchored starting states that is consistent
        // with our DFA's configuration. Unconditionally adding both (although
        // it is the default) can make DFAs quite a bit bigger.
        if self.dfa.start_kind().has_unanchored() {
            self.add_start_group(Anchored::No, dfa_state_ids)?;
        }
        if self.dfa.start_kind().has_anchored() {
            self.add_start_group(Anchored::Yes, dfa_state_ids)?;
        }
        // I previously has an 'assert' here checking that either
        // 'dfa_state_ids' was non-empty, or the NFA had zero patterns. But it
        // turns out this isn't always true. For example, the NFA might have
        // one or more patterns but where all such patterns are just 'fail'
        // states. These will ultimately just compile down to DFA dead states,
        // and since the dead state was added earlier, no new DFA states are
        // added. And thus, it is valid and okay for 'dfa_state_ids' to be
        // empty even if there are a non-zero number of patterns in the NFA.

        // We only need to compute anchored start states for each pattern if it
        // was requested to do so.
        if self.dfa.starts_for_each_pattern() {
            for pid in self.nfa.patterns() {
                self.add_start_group(Anchored::Pattern(pid), dfa_state_ids)?;
            }
        }
        Ok(())
    }