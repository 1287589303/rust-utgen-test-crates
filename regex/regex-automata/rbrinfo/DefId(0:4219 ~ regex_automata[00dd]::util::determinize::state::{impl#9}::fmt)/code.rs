fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut nfa_ids = alloc::vec![];
        self.iter_nfa_state_ids(|sid| nfa_ids.push(sid));
        f.debug_struct("Repr")
            .field("is_match", &self.is_match())
            .field("is_from_word", &self.is_from_word())
            .field("is_half_crlf", &self.is_half_crlf())
            .field("look_have", &self.look_have())
            .field("look_need", &self.look_need())
            .field("match_pattern_ids", &self.match_pattern_ids())
            .field("nfa_state_ids", &nfa_ids)
            .finish()
    }