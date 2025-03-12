fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool {
        if let Some(e) = self.dfa.get(input) {
            trace!(
                "using full DFA for is-match search at {:?}",
                input.get_span()
            );
            match e.try_search_half_fwd(input) {
                Ok(x) => x.is_some(),
                Err(_err) => {
                    trace!("full DFA half search failed: {}", _err);
                    self.is_match_nofail(cache, input)
                }
            }
        } else if let Some(e) = self.hybrid.get(input) {
            trace!(
                "using lazy DFA for is-match search at {:?}",
                input.get_span()
            );
            match e.try_search_half_fwd(&mut cache.hybrid, input) {
                Ok(x) => x.is_some(),
                Err(_err) => {
                    trace!("lazy DFA half search failed: {}", _err);
                    self.is_match_nofail(cache, input)
                }
            }
        } else {
            self.is_match_nofail(cache, input)
        }
    }