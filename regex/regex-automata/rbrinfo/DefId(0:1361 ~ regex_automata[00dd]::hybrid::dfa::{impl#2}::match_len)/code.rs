pub fn match_len(&self, cache: &Cache, id: LazyStateID) -> usize {
        assert!(id.is_match());
        LazyRef::new(self, cache).get_cached_state(id).match_len()
    }