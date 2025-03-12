fn stack_push(
        &mut self,
        nfa_id: StateID,
        epsilons: Epsilons,
    ) -> Result<(), BuildError> {
        // If we already have seen a match and we are compiling a leftmost
        // first DFA, then we shouldn't add any more states to look at. This is
        // effectively how preference order and non-greediness is implemented.
        // if !self.config.get_match_kind().continue_past_first_match()
        // && self.matched
        // {
        // return Ok(());
        // }
        if !self.seen.insert(nfa_id) {
            return Err(BuildError::not_one_pass(
                "multiple epsilon transitions to same state",
            ));
        }
        self.stack.push((nfa_id, epsilons));
        Ok(())
    }