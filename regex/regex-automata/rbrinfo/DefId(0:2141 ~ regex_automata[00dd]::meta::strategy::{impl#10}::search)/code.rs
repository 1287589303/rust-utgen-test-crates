fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match> {
        if input.get_anchored().is_anchored() {
            return self.core.search(cache, input);
        }
        match self.try_search_full(cache, input) {
            Err(RetryError::Quadratic(_err)) => {
                trace!("reverse inner optimization failed: {}", _err);
                self.core.search(cache, input)
            }
            Err(RetryError::Fail(_err)) => {
                trace!("reverse inner fast search failed: {}", _err);
                self.core.search_nofail(cache, input)
            }
            Ok(matornot) => matornot,
        }
    }