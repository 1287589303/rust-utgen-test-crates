pub fn search_slots_with(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        if self.imp.info.is_impossible(input) {
            return None;
        }
        self.imp.strat.search_slots(cache, input, slots)
    }