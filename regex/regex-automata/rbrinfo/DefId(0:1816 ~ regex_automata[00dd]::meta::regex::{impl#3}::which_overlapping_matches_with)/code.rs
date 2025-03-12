pub fn which_overlapping_matches_with(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {
        if self.imp.info.is_impossible(input) {
            return;
        }
        self.imp.strat.which_overlapping_matches(cache, input, patset)
    }