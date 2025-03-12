pub(crate) fn into_nfa(mut self) -> StateBuilderNFA {
        self.repr_vec().close_match_pattern_ids();
        StateBuilderNFA { repr: self.0, prev_nfa_state_id: StateID::ZERO }
    }