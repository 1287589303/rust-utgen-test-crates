pub fn search_half(&self, input: &Input<'_>) -> Option<HalfMatch> {
        if self.imp.info.is_impossible(input) {
            return None;
        }
        let mut guard = self.pool.get();
        let result = self.imp.strat.search_half(&mut guard, input);
        // See 'Regex::search' for why we put the guard back explicitly.
        PoolGuard::put(guard);
        result
    }