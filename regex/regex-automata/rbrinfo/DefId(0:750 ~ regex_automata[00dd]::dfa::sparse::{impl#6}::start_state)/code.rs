fn start_state(
        &self,
        config: &start::Config,
    ) -> Result<StateID, StartError> {
        let anchored = config.get_anchored();
        let start = match config.get_look_behind() {
            None => Start::Text,
            Some(byte) => {
                if !self.quitset.is_empty() && self.quitset.contains(byte) {
                    return Err(StartError::quit(byte));
                }
                self.st.start_map.get(byte)
            }
        };
        self.st.start(anchored, start)
    }