fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match> {
        if input.get_anchored().is_anchored() {
            return self.core.search(cache, input);
        }
        match self.try_search_half_start(cache, input) {
            Err(RetryError::Quadratic(_err)) => {
                trace!("reverse suffix optimization failed: {}", _err);
                self.core.search(cache, input)
            }
            Err(RetryError::Fail(_err)) => {
                trace!("reverse suffix reverse fast search failed: {}", _err);
                self.core.search_nofail(cache, input)
            }
            Ok(None) => None,
            Ok(Some(hm_start)) => {
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
                        self.core.search_nofail(cache, input)
                    }
                    Ok(None) => {
                        unreachable!(
                            "suffix match plus reverse match implies \
						     there must be a match",
                        )
                    }
                    Ok(Some(hm_end)) => Some(Match::new(
                        hm_start.pattern(),
                        hm_start.offset()..hm_end.offset(),
                    )),
                }
            }
        }
    }