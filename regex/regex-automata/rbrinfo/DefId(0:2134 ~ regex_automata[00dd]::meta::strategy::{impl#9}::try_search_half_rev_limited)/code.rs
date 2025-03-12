fn try_search_half_rev_limited(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {
        if let Some(e) = self.dfa.get(&input) {
            trace!(
                "using full DFA for reverse inner search at {:?}, \
                 but will be stopped at {} to avoid quadratic behavior",
                input.get_span(),
                min_start,
            );
            e.try_search_half_rev_limited(&input, min_start)
        } else if let Some(e) = self.hybrid.get(&input) {
            trace!(
                "using lazy DFA for reverse inner search at {:?}, \
                 but will be stopped at {} to avoid quadratic behavior",
                input.get_span(),
                min_start,
            );
            e.try_search_half_rev_limited(
                &mut cache.revhybrid,
                &input,
                min_start,
            )
        } else {
            unreachable!("ReverseInner always has a DFA")
        }
    }