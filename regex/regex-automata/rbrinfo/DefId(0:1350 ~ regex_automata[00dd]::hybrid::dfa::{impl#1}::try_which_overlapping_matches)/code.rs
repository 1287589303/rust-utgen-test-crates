pub fn try_which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), MatchError> {
        let mut state = OverlappingState::start();
        while let Some(m) = {
            self.try_search_overlapping_fwd(cache, input, &mut state)?;
            state.get_match()
        } {
            let _ = patset.try_insert(m.pattern());
            // There's nothing left to find, so we can stop. Or the caller
            // asked us to.
            if patset.is_full() || input.get_earliest() {
                break;
            }
        }
        Ok(())
    }