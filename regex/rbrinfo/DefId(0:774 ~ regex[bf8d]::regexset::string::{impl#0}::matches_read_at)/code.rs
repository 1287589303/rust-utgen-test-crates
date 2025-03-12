pub fn matches_read_at(
        &self,
        matches: &mut [bool],
        haystack: &str,
        start: usize,
    ) -> bool {
        // This is pretty dumb. We should try to fix this, but the
        // regex-automata API doesn't provide a way to store matches in an
        // arbitrary &mut [bool]. Thankfully, this API is doc(hidden) and
        // thus not public... But regex-capi currently uses it. We should
        // fix regex-capi to use a PatternSet, maybe? Not sure... PatternSet
        // is in regex-automata, not regex. So maybe we should just accept a
        // 'SetMatches', which is basically just a newtype around PatternSet.
        let mut patset = PatternSet::new(self.meta.pattern_len());
        let mut input = Input::new(haystack);
        input.set_start(start);
        self.meta.which_overlapping_matches(&input, &mut patset);
        for pid in patset.iter() {
            matches[pid] = true;
        }
        !patset.is_empty()
    }