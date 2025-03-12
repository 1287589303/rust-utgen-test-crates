fn dead(
        kind: StartKind,
        lookm: &LookMatcher,
        pattern_len: Option<usize>,
    ) -> Result<StartTable<Vec<u32>>, BuildError> {
        if let Some(len) = pattern_len {
            assert!(len <= PatternID::LIMIT);
        }
        let stride = Start::len();
        // OK because 2*4 is never going to overflow anything.
        let starts_len = stride.checked_mul(2).unwrap();
        let pattern_starts_len =
            match stride.checked_mul(pattern_len.unwrap_or(0)) {
                Some(x) => x,
                None => return Err(BuildError::too_many_start_states()),
            };
        let table_len = match starts_len.checked_add(pattern_starts_len) {
            Some(x) => x,
            None => return Err(BuildError::too_many_start_states()),
        };
        if let Err(_) = isize::try_from(table_len) {
            return Err(BuildError::too_many_start_states());
        }
        let table = vec![DEAD.as_u32(); table_len];
        let start_map = StartByteMap::new(lookm);
        Ok(StartTable {
            table,
            kind,
            start_map,
            stride,
            pattern_len,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        })
    }