fn add_start_state(
        &mut self,
        pid: Option<PatternID>,
        nfa_id: StateID,
    ) -> Result<StateID, BuildError> {
        match pid {
            // With no pid, this should be the start state for all patterns
            // and thus be the first one.
            None => assert!(self.dfa.starts.is_empty()),
            // With a pid, we want it to be at self.dfa.starts[pid+1].
            Some(pid) => assert!(self.dfa.starts.len() == pid.one_more()),
        }
        let dfa_id = self.add_dfa_state_for_nfa_state(nfa_id)?;
        self.dfa.starts.push(dfa_id);
        Ok(dfa_id)
    }