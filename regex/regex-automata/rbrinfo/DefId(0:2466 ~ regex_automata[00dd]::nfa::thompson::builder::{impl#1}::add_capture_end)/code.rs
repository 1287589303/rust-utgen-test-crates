pub fn add_capture_end(
        &mut self,
        next: StateID,
        group_index: u32,
    ) -> Result<StateID, BuildError> {
        let pid = self.current_pattern_id();
        let group_index = match SmallIndex::try_from(group_index) {
            Err(_) => {
                return Err(BuildError::invalid_capture_index(group_index))
            }
            Ok(group_index) => group_index,
        };
        self.add(State::CaptureEnd { pattern_id: pid, group_index, next })
    }