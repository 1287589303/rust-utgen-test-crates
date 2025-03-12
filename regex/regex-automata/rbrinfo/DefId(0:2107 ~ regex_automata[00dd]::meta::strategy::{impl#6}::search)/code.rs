fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match> {
        if input.get_anchored().is_anchored() {
            return self.core.search(cache, input);
        }
        match self.try_search_half_anchored_rev(cache, input) {
            Err(_err) => {
                trace!("fast reverse anchored search failed: {}", _err);
                self.core.search_nofail(cache, input)
            }
            Ok(None) => None,
            Ok(Some(hm)) => {
                Some(Match::new(hm.pattern(), hm.offset()..input.end()))
            }
        }
    }