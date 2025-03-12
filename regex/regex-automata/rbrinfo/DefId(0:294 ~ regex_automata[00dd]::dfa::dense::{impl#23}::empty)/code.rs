fn empty(pattern_len: usize) -> MatchStates<Vec<u32>> {
        assert!(pattern_len <= PatternID::LIMIT);
        MatchStates { slices: vec![], pattern_ids: vec![], pattern_len }
    }