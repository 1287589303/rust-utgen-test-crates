fn search_half(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<HalfMatch> {
        if input.get_anchored().is_anchored() {
            return self.core.search_half(cache, input);
        }
        match self.try_search_full(cache, input) {
            Err(RetryError::Quadratic(_err)) => {
                trace!("reverse inner half optimization failed: {}", _err);
                self.core.search_half(cache, input)
            }
            Err(RetryError::Fail(_err)) => {
                trace!("reverse inner fast half search failed: {}", _err);
                self.core.search_half_nofail(cache, input)
            }
            Ok(None) => None,
            Ok(Some(m)) => Some(HalfMatch::new(m.pattern(), m.end())),
        }
    }