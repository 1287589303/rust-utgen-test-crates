fn search_half(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<HalfMatch> {
        if input.get_anchored().is_anchored() {
            return self.core.search_half(cache, input);
        }
        match self.try_search_half_start(cache, input) {
            Err(RetryError::Quadratic(_err)) => {
                trace!("reverse suffix half optimization failed: {}", _err);
                self.core.search_half(cache, input)
            }
            Err(RetryError::Fail(_err)) => {
                trace!(
                    "reverse suffix reverse fast half search failed: {}",
                    _err
                );
                self.core.search_half_nofail(cache, input)
            }
            Ok(None) => None,
            Ok(Some(hm_start)) => {
                // This is a bit subtle. It is tempting to just stop searching
                // at this point and return a half-match with an offset
                // corresponding to where the suffix was found. But the suffix
                // match does not necessarily correspond to the end of the
                // proper leftmost-first match. Consider /[a-z]+ing/ against
                // 'tingling'. The first suffix match is the first 'ing', and
                // the /[a-z]+/ matches the 't'. So if we stopped here, then
                // we'd report 'ting' as the match. But 'tingling' is the
                // correct match because of greediness.
                let fwdinput = input
                    .clone()
                    .anchored(Anchored::Pattern(hm_start.pattern()))
                    .span(hm_start.offset()..input.end());
                match self.try_search_half_fwd(cache, &fwdinput) {
                    Err(_err) => {
                        trace!(
                            "reverse suffix forward fast search failed: {}",
                            _err
                        );
                        self.core.search_half_nofail(cache, input)
                    }
                    Ok(None) => {
                        unreachable!(
                            "suffix match plus reverse match implies \
						     there must be a match",
                        )
                    }
                    Ok(Some(hm_end)) => Some(hm_end),
                }
            }
        }
    }