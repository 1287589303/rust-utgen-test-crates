pub fn start_state_reverse(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<LazyStateID, MatchError> {
        let config = start::Config::from_input_reverse(input);
        self.start_state(cache, &config).map_err(|err| match err {
            StartError::Cache { .. } => MatchError::gave_up(input.end()),
            StartError::Quit { byte } => {
                let offset = input.end();
                MatchError::quit(byte, offset)
            }
            StartError::UnsupportedAnchored { mode } => {
                MatchError::unsupported_anchored(mode)
            }
        })
    }