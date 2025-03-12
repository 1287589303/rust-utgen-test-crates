fn search(&self, _cache: &mut Cache, input: &Input<'_>) -> Option<Match> {
        if input.is_done() {
            return None;
        }
        if input.get_anchored().is_anchored() {
            return self
                .pre
                .prefix(input.haystack(), input.get_span())
                .map(|sp| Match::new(PatternID::ZERO, sp));
        }
        self.pre
            .find(input.haystack(), input.get_span())
            .map(|sp| Match::new(PatternID::ZERO, sp))
    }