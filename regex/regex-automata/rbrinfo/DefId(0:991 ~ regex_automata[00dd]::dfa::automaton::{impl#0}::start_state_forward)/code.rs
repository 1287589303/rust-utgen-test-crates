fn start_state_forward(
        &self,
        input: &Input<'_>,
    ) -> Result<StateID, MatchError> {
        (**self).start_state_forward(input)
    }