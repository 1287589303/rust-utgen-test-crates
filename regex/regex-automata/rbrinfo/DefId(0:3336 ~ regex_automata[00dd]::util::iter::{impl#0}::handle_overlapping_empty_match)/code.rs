fn handle_overlapping_empty_match<F>(
        &mut self,
        m: Match,
        mut finder: F,
    ) -> Result<Option<Match>, MatchError>
    where
        F: FnMut(&Input<'_>) -> Result<Option<Match>, MatchError>,
    {
        assert!(m.is_empty());
        self.input.set_start(self.input.start().checked_add(1).unwrap());
        finder(&self.input)
    }