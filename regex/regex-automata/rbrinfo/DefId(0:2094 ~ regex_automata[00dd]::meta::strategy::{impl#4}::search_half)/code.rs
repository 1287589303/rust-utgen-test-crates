fn search_half(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<HalfMatch> {
        // The main difference with 'search' is that if we're using a DFA, we
        // can use a single forward scan without needing to run the reverse
        // DFA.
        if let Some(e) = self.dfa.get(input) {
            trace!("using full DFA for half search at {:?}", input.get_span());
            match e.try_search_half_fwd(input) {
                Ok(x) => x,
                Err(_err) => {
                    trace!("full DFA half search failed: {}", _err);
                    self.search_half_nofail(cache, input)
                }
            }
        } else if let Some(e) = self.hybrid.get(input) {
            trace!("using lazy DFA for half search at {:?}", input.get_span());
            match e.try_search_half_fwd(&mut cache.hybrid, input) {
                Ok(x) => x,
                Err(_err) => {
                    trace!("lazy DFA half search failed: {}", _err);
                    self.search_half_nofail(cache, input)
                }
            }
        } else {
            self.search_half_nofail(cache, input)
        }
    }