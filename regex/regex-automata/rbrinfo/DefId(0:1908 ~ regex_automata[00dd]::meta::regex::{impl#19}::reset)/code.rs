pub fn reset(&mut self, re: &Regex) {
        re.imp.strat.reset_cache(self)
    }