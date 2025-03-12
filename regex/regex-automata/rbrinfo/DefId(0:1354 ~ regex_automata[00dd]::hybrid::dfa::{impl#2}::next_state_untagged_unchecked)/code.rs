pub unsafe fn next_state_untagged_unchecked(
        &self,
        cache: &Cache,
        current: LazyStateID,
        input: u8,
    ) -> LazyStateID {
        debug_assert!(!current.is_tagged());
        let class = usize::from(self.classes.get(input));
        let offset = current.as_usize_unchecked() + class;
        *cache.trans.get_unchecked(offset)
    }