fn from_dense_dfa<T: AsRef<[u32]>>(
        dfa: &dense::DFA<T>,
        remap: &[StateID],
    ) -> Result<StartTable<Vec<u8>>, BuildError> {
        // Unless the DFA has start states compiled for each pattern, then
        // as far as the starting state table is concerned, there are zero
        // patterns to account for. It will instead only store starting states
        // for the entire DFA.
        let start_pattern_len = if dfa.starts_for_each_pattern() {
            Some(dfa.pattern_len())
        } else {
            None
        };
        let mut sl = StartTable::new(dfa, start_pattern_len);
        for (old_start_id, anchored, sty) in dfa.starts() {
            let new_start_id = remap[dfa.to_index(old_start_id)];
            sl.set_start(anchored, sty, new_start_id);
        }
        Ok(sl)
    }