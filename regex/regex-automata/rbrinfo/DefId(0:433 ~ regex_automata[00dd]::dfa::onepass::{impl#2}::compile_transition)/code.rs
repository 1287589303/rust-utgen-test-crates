fn compile_transition(
        &mut self,
        dfa_id: StateID,
        trans: &thompson::Transition,
        epsilons: Epsilons,
    ) -> Result<(), BuildError> {
        let next_dfa_id = self.add_dfa_state_for_nfa_state(trans.next)?;
        for byte in self
            .classes
            .representatives(trans.start..=trans.end)
            .filter_map(|r| r.as_u8())
        {
            let oldtrans = self.dfa.transition(dfa_id, byte);
            let newtrans =
                Transition::new(self.matched, next_dfa_id, epsilons);
            // If the old transition points to the DEAD state, then we know
            // 'byte' has not been mapped to any transition for this DFA state
            // yet. So set it unconditionally. Otherwise, we require that the
            // old and new transitions are equivalent. Otherwise, there is
            // ambiguity and thus the regex is not one-pass.
            if oldtrans.state_id() == DEAD {
                self.dfa.set_transition(dfa_id, byte, newtrans);
            } else if oldtrans != newtrans {
                return Err(BuildError::not_one_pass(
                    "conflicting transition",
                ));
            }
        }
        Ok(())
    }