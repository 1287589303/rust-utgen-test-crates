fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match> {
        // We manually inline try_search_mayfail here because letting the
        // compiler do it seems to produce pretty crappy codegen.
        return if let Some(e) = self.dfa.get(input) {
            trace!("using full DFA for full search at {:?}", input.get_span());
            match e.try_search(input) {
                Ok(x) => x,
                Err(_err) => {
                    trace!("full DFA search failed: {}", _err);
                    self.search_nofail(cache, input)
                }
            }
        } else if let Some(e) = self.hybrid.get(input) {
            trace!("using lazy DFA for full search at {:?}", input.get_span());
            match e.try_search(&mut cache.hybrid, input) {
                Ok(x) => x,
                Err(_err) => {
                    trace!("lazy DFA search failed: {}", _err);
                    self.search_nofail(cache, input)
                }
            }
        } else {
            self.search_nofail(cache, input)
        };
    }