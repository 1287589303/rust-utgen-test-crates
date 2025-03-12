fn search_half(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<HalfMatch> {
        self.search(cache, input).map(|m| HalfMatch::new(m.pattern(), m.end()))
    }