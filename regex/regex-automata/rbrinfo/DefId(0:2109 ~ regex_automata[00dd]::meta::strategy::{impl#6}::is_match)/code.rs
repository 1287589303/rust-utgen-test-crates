fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool {
        if input.get_anchored().is_anchored() {
            return self.core.is_match(cache, input);
        }
        match self.try_search_half_anchored_rev(cache, input) {
            Err(_err) => {
                trace!("fast reverse anchored search failed: {}", _err);
                self.core.is_match_nofail(cache, input)
            }
            Ok(None) => false,
            Ok(Some(_)) => true,
        }
    }