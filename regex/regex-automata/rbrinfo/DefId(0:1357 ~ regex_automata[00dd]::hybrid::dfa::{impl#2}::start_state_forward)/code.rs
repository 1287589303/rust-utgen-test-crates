pub fn start_state_forward(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<LazyStateID, MatchError> {
        let config = start::Config::from_input_forward(input);
        self.start_state(cache, &config).map_err(|err| match err {
            StartError::Cache { .. } => MatchError::gave_up(input.start()),
            StartError::Quit { byte } => {
                let offset = input
                    .start()
                    .checked_sub(1)
                    .expect("no quit in start without look-behind");
                MatchError::quit(byte, offset)
            }
            StartError::UnsupportedAnchored { mode } => {
                MatchError::unsupported_anchored(mode)
            }
        })
    }