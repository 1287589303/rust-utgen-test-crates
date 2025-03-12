fn handle_overlapping_empty_half_match<F>(
        &mut self,
        _: HalfMatch,
        mut finder: F,
    ) -> Result<Option<HalfMatch>, MatchError>
    where
        F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
    {
        // Since we are only here when 'm.offset()' matches the offset of the
        // last match, it follows that this must have been an empty match.
        // Since we both need to make progress *and* prevent overlapping
        // matches, we discard this match and advance the search by 1.
        //
        // Note that this may start a search in the middle of a codepoint. The
        // regex engines themselves are expected to deal with that and not
        // report any matches within a codepoint if they are configured in
        // UTF-8 mode.
        self.input.set_start(self.input.start().checked_add(1).unwrap());
        finder(&self.input)
    }