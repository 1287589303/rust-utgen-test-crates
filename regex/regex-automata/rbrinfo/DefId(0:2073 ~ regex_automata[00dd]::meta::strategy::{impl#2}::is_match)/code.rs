fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool {
        self.search(cache, input).is_some()
    }