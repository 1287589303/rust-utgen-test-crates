fn start_state(
        &self,
        config: &start::Config,
    ) -> Result<StateID, StartError> {
        (**self).start_state(config)
    }