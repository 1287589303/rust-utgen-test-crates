fn try_search_half_anchored_rev(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryFailError> {
        // We of course always want an anchored search. In theory, the
        // underlying regex engines should automatically enable anchored
        // searches since the regex is itself anchored, but this more clearly
        // expresses intent and is always correct.
        let input = input.clone().anchored(Anchored::Yes);
        if let Some(e) = self.core.dfa.get(&input) {
            trace!(
                "using full DFA for reverse anchored search at {:?}",
                input.get_span()
            );
            e.try_search_half_rev(&input)
        } else if let Some(e) = self.core.hybrid.get(&input) {
            trace!(
                "using lazy DFA for reverse anchored search at {:?}",
                input.get_span()
            );
            e.try_search_half_rev(&mut cache.hybrid, &input)
        } else {
            unreachable!("ReverseAnchored always has a DFA")
        }
    }