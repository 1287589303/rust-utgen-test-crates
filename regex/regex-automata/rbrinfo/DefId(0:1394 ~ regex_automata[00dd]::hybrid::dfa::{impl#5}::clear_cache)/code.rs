fn clear_cache(&mut self) {
        self.cache.trans.clear();
        self.cache.starts.clear();
        self.cache.states.clear();
        self.cache.states_to_id.clear();
        self.cache.memory_usage_state = 0;
        self.cache.clear_count += 1;
        self.cache.bytes_searched = 0;
        if let Some(ref mut progress) = self.cache.progress {
            progress.start = progress.at;
        }
        trace!(
            "lazy DFA cache has been cleared (count: {})",
            self.cache.clear_count
        );
        self.init_cache();
        // If the state we want to save is one of the sentinel
        // (unknown/dead/quit) states, then 'init_cache' adds those back, and
        // their identifier values remains invariant. So there's no need to add
        // it again. (And indeed, doing so would be incorrect!)
        if let Some((old_id, state)) = self.cache.state_saver.take_to_save() {
            // If the state is one of the special sentinel states, then it is
            // automatically added by cache initialization and its ID always
            // remains the same. With that said, this should never occur since
            // the sentinel states are all loop states back to themselves. So
            // we should never be in a position where we're attempting to save
            // a sentinel state since we never compute transitions out of a
            // sentinel state.
            assert!(
                !self.as_ref().is_sentinel(old_id),
                "cannot save sentinel state"
            );
            let new_id = self
                .add_state(state, |id| {
                    if old_id.is_start() {
                        // We don't need to consult the
                        // 'specialize_start_states' config knob here, because
                        // if it's disabled, old_id.is_start() will never
                        // return true.
                        id.to_start()
                    } else {
                        id
                    }
                })
                // The unwrap here is OK because lazy DFA creation ensures that
                // we have room in the cache to add MIN_STATES states. Since
                // 'init_cache' above adds 3, this adds a 4th.
                .expect("adding one state after cache clear must work");
            self.cache.state_saver = StateSaver::Saved(new_id);
        }
    }