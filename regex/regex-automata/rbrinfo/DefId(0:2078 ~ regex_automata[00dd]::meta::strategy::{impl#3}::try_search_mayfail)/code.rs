fn try_search_mayfail(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<Result<Option<Match>, RetryFailError>> {
        if let Some(e) = self.dfa.get(input) {
            trace!("using full DFA for search at {:?}", input.get_span());
            Some(e.try_search(input))
        } else if let Some(e) = self.hybrid.get(input) {
            trace!("using lazy DFA for search at {:?}", input.get_span());
            Some(e.try_search(&mut cache.hybrid, input))
        } else {
            None
        }
    }