fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {
        if !self.is_match() {
            return None;
        }
        let mut pids = alloc::vec![];
        self.iter_match_pattern_ids(|pid| pids.push(pid));
        Some(pids)
    }