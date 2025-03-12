fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool {
        if input.get_anchored().is_anchored() {
            return self.core.is_match(cache, input);
        }
        match self.try_search_half_start(cache, input) {
            Err(RetryError::Quadratic(_err)) => {
                trace!("reverse suffix half optimization failed: {}", _err);
                self.core.is_match_nofail(cache, input)
            }
            Err(RetryError::Fail(_err)) => {
                trace!(
                    "reverse suffix reverse fast half search failed: {}",
                    _err
                );
                self.core.is_match_nofail(cache, input)
            }
            Ok(None) => false,
            Ok(Some(_)) => true,
        }
    }