pub fn which_overlapping_matches(
        &self,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {
        if self.imp.info.is_impossible(input) {
            return;
        }
        let mut guard = self.pool.get();
        let result = self
            .imp
            .strat
            .which_overlapping_matches(&mut guard, input, patset);
        // See 'Regex::search' for why we put the guard back explicitly.
        PoolGuard::put(guard);
        result
    }