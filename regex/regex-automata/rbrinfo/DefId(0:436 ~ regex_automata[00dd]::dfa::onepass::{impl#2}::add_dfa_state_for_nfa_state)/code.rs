fn add_dfa_state_for_nfa_state(
        &mut self,
        nfa_id: StateID,
    ) -> Result<StateID, BuildError> {
        // If we've already built a DFA state for the given NFA state, then
        // just return that. We definitely do not want to have more than one
        // DFA state in existence for the same NFA state, since all but one of
        // them will likely become unreachable. And at least some of them are
        // likely to wind up being incomplete.
        let existing_dfa_id = self.nfa_to_dfa_id[nfa_id];
        if existing_dfa_id != DEAD {
            return Ok(existing_dfa_id);
        }
        // If we don't have any DFA state yet, add it and then add the given
        // NFA state to the list of states to explore.
        let dfa_id = self.add_empty_state()?;
        self.nfa_to_dfa_id[nfa_id] = dfa_id;
        self.uncompiled_nfa_ids.push(nfa_id);
        Ok(dfa_id)
    }