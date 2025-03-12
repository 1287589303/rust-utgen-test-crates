fn new<T: AsRef<[u32]>>(
        dfa: &dense::DFA<T>,
        pattern_len: Option<usize>,
    ) -> StartTable<Vec<u8>> {
        let stride = Start::len();
        // This is OK since the only way we're here is if a dense DFA could be
        // constructed successfully, which uses the same space.
        let len = stride
            .checked_mul(pattern_len.unwrap_or(0))
            .unwrap()
            .checked_add(stride.checked_mul(2).unwrap())
            .unwrap()
            .checked_mul(StateID::SIZE)
            .unwrap();
        StartTable {
            table: vec![0; len],
            kind: dfa.start_kind(),
            start_map: dfa.start_map().clone(),
            stride,
            pattern_len,
            universal_start_unanchored: dfa
                .universal_start_state(Anchored::No),
            universal_start_anchored: dfa.universal_start_state(Anchored::Yes),
        }
    }