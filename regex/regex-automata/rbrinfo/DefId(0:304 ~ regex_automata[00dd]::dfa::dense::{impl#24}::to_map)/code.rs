fn to_map(&self, dfa: &DFA<T>) -> BTreeMap<StateID, Vec<PatternID>> {
        let mut map = BTreeMap::new();
        for i in 0..self.len() {
            let mut pids = vec![];
            for j in 0..self.pattern_len(i) {
                pids.push(self.pattern_id(i, j));
            }
            map.insert(self.match_state_id(dfa, i), pids);
        }
        map
    }