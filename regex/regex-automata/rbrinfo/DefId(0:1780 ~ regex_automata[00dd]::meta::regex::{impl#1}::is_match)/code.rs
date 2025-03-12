pub fn is_match<'h, I: Into<Input<'h>>>(&self, input: I) -> bool {
        let input = input.into().earliest(true);
        if self.imp.info.is_impossible(&input) {
            return false;
        }
        let mut guard = self.pool.get();
        let result = self.imp.strat.is_match(&mut guard, &input);
        // See 'Regex::search' for why we put the guard back explicitly.
        PoolGuard::put(guard);
        result
    }