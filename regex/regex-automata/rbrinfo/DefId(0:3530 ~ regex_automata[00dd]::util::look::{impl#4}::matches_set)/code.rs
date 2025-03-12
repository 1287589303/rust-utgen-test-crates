pub fn matches_set(
        &self,
        set: LookSet,
        haystack: &[u8],
        at: usize,
    ) -> bool {
        self.matches_set_inline(set, haystack, at)
    }