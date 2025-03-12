pub fn start_state(
        &self,
        cache: &mut Cache,
        config: &start::Config,
    ) -> Result<LazyStateID, StartError> {
        let lazy = LazyRef::new(self, cache);
        let anchored = config.get_anchored();
        let start = match config.get_look_behind() {
            None => Start::Text,
            Some(byte) => {
                if !self.quitset.is_empty() && self.quitset.contains(byte) {
                    return Err(StartError::quit(byte));
                }
                self.start_map.get(byte)
            }
        };
        let start_id = lazy.get_cached_start_id(anchored, start)?;
        if !start_id.is_unknown() {
            return Ok(start_id);
        }
        Lazy::new(self, cache).cache_start_group(anchored, start)
    }