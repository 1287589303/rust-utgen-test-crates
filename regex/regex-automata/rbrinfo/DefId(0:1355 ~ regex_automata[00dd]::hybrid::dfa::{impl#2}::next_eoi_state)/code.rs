pub fn next_eoi_state(
        &self,
        cache: &mut Cache,
        current: LazyStateID,
    ) -> Result<LazyStateID, CacheError> {
        let eoi = self.classes.eoi().as_usize();
        let offset = current.as_usize_untagged() + eoi;
        let sid = cache.trans[offset];
        if !sid.is_unknown() {
            return Ok(sid);
        }
        let unit = self.classes.eoi();
        Lazy::new(self, cache).cache_next_state(current, unit)
    }