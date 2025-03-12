fn new(
        matches: &BTreeMap<StateID, Vec<PatternID>>,
        pattern_len: usize,
    ) -> Result<MatchStates<Vec<u32>>, BuildError> {
        let mut m = MatchStates::empty(pattern_len);
        for (_, pids) in matches.iter() {
            let start = PatternID::new(m.pattern_ids.len())
                .map_err(|_| BuildError::too_many_match_pattern_ids())?;
            m.slices.push(start.as_u32());
            // This is always correct since the number of patterns in a single
            // match state can never exceed maximum number of allowable
            // patterns. Why? Because a pattern can only appear once in a
            // particular match state, by construction. (And since our pattern
            // ID limit is one less than u32::MAX, we're guaranteed that the
            // length fits in a u32.)
            m.slices.push(u32::try_from(pids.len()).unwrap());
            for &pid in pids {
                m.pattern_ids.push(pid.as_u32());
            }
        }
        m.pattern_len = pattern_len;
        Ok(m)
    }