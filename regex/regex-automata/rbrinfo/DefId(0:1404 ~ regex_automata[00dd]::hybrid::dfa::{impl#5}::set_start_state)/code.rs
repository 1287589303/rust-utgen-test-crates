fn set_start_state(
        &mut self,
        anchored: Anchored,
        start: Start,
        id: LazyStateID,
    ) {
        assert!(self.as_ref().is_valid(id));
        let start_index = start.as_usize();
        let index = match anchored {
            Anchored::No => start_index,
            Anchored::Yes => Start::len() + start_index,
            Anchored::Pattern(pid) => {
                assert!(
                    self.dfa.get_config().get_starts_for_each_pattern(),
                    "attempted to search for a specific pattern \
                     without enabling starts_for_each_pattern",
                );
                let pid = pid.as_usize();
                (2 * Start::len()) + (Start::len() * pid) + start_index
            }
        };
        self.cache.starts[index] = id;
    }