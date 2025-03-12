pub fn try_advance_half<F>(
        &mut self,
        mut finder: F,
    ) -> Result<Option<HalfMatch>, MatchError>
    where
        F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
    {
        let mut m = match finder(&self.input)? {
            None => return Ok(None),
            Some(m) => m,
        };
        if Some(m.offset()) == self.last_match_end {
            m = match self.handle_overlapping_empty_half_match(m, finder)? {
                None => return Ok(None),
                Some(m) => m,
            };
        }
        self.input.set_start(m.offset());
        self.last_match_end = Some(m.offset());
        Ok(Some(m))
    }