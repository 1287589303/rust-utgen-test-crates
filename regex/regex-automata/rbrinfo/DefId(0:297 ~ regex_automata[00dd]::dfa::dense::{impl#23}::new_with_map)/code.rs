fn new_with_map(
        &self,
        matches: &BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<MatchStates<Vec<u32>>, BuildError> {
        MatchStates::new(matches, self.pattern_len)
    }