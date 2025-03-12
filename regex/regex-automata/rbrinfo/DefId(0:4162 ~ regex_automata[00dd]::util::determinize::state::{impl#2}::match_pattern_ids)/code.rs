pub(crate) fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {
        self.repr().match_pattern_ids()
    }