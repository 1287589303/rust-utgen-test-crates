fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool {
        if input.get_anchored().is_anchored() {
            return self.core.is_match(cache, input);
        }
        match self.try_search_full(cache, input) {
            Err(RetryError::Quadratic(_err)) => {
                trace!("reverse inner half optimization failed: {}", _err);
                self.core.is_match_nofail(cache, input)
            }
            Err(RetryError::Fail(_err)) => {
                trace!("reverse inner fast half search failed: {}", _err);
                self.core.is_match_nofail(cache, input)
            }
            Ok(None) => false,
            Ok(Some(_)) => true,
        }
    }