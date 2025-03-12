fn start_state_reverse(
        &self,
        input: &Input<'_>,
    ) -> Result<StateID, MatchError> {
        let config = start::Config::from_input_reverse(input);
        self.start_state(&config).map_err(|err| match err {
            StartError::Quit { byte } => {
                let offset = input.end();
                MatchError::quit(byte, offset)
            }
            StartError::UnsupportedAnchored { mode } => {
                MatchError::unsupported_anchored(mode)
            }
        })
    }