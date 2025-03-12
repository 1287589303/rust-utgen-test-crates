pub(crate) fn which_overlapping_matches(
        &self,
        cache: &mut PikeVMCache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {
        self.0.which_overlapping_matches(
            cache.0.as_mut().unwrap(),
            input,
            patset,
        )
    }