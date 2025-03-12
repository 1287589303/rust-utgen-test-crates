pub fn search_with(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<Match> {
        if self.imp.info.is_impossible(input) {
            return None;
        }
        self.imp.strat.search(cache, input)
    }