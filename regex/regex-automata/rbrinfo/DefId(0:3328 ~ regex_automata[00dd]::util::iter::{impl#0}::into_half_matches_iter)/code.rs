pub fn into_half_matches_iter<F>(
        self,
        finder: F,
    ) -> TryHalfMatchesIter<'h, F>
    where
        F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
    {
        TryHalfMatchesIter { it: self, finder }
    }