fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {
        // It seems like this could probably benefit from a reverse anchored
        // optimization, perhaps by doing an overlapping reverse search (which
        // the DFAs do support). I haven't given it much thought though, and
        // I'm currently focus more on the single pattern case.
        self.core.which_overlapping_matches(cache, input, patset)
    }