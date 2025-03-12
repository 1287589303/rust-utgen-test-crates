pub fn try_search_overlapping_fwd(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError> {
        let utf8empty = self.get_nfa().has_empty() && self.get_nfa().is_utf8();
        search::find_overlapping_fwd(self, cache, input, state)?;
        match state.get_match() {
            None => Ok(()),
            Some(_) if !utf8empty => Ok(()),
            Some(_) => skip_empty_utf8_splits_overlapping(
                input,
                state,
                |input, state| {
                    search::find_overlapping_fwd(self, cache, input, state)
                },
            ),
        }
    }