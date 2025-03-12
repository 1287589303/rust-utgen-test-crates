fn start_state_reverse(
        &self,
        input: &Input<'_>,
    ) -> Result<StateID, MatchError> {
        (**self).start_state_reverse(input)
    }