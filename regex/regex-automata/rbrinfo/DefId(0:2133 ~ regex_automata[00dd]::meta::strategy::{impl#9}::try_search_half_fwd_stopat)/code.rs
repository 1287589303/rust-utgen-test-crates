fn try_search_half_fwd_stopat(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Result<HalfMatch, usize>, RetryFailError> {
        if let Some(e) = self.core.dfa.get(&input) {
            trace!(
                "using full DFA for forward reverse inner search at {:?}",
                input.get_span()
            );
            e.try_search_half_fwd_stopat(&input)
        } else if let Some(e) = self.core.hybrid.get(&input) {
            trace!(
                "using lazy DFA for forward reverse inner search at {:?}",
                input.get_span()
            );
            e.try_search_half_fwd_stopat(&mut cache.hybrid, &input)
        } else {
            unreachable!("ReverseInner always has a DFA")
        }
    }