fn search_half(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<HalfMatch> {
        if input.get_anchored().is_anchored() {
            return self.core.search_half(cache, input);
        }
        match self.try_search_half_anchored_rev(cache, input) {
            Err(_err) => {
                trace!("fast reverse anchored search failed: {}", _err);
                self.core.search_half_nofail(cache, input)
            }
            Ok(None) => None,
            Ok(Some(hm)) => {
                // Careful here! 'try_search_half' is a *forward* search that
                // only cares about the *end* position of a match. But
                // 'hm.offset()' is actually the start of the match. So we
                // actually just throw that away here and, since we know we
                // have a match, return the only possible position at which a
                // match can occur: input.end().
                Some(HalfMatch::new(hm.pattern(), input.end()))
            }
        }
    }