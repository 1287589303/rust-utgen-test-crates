fn save_state(&mut self, id: LazyStateID) {
        let state = self.as_ref().get_cached_state(id).clone();
        self.cache.state_saver = StateSaver::ToSave { id, state };
    }