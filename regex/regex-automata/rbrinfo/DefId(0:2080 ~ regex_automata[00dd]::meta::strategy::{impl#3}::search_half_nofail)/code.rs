fn search_half_nofail(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<HalfMatch> {
        // Only the lazy/full DFA returns half-matches, since the DFA requires
        // a reverse scan to find the start position. These fallback regex
        // engines can find the start and end in a single pass, so we just do
        // that and throw away the start offset to conform to the API.
        let m = self.search_nofail(cache, input)?;
        Some(HalfMatch::new(m.pattern(), m.end()))
    }