pub fn search_half_with(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<HalfMatch> {
        if self.imp.info.is_impossible(input) {
            return None;
        }
        self.imp.strat.search_half(cache, input)
    }