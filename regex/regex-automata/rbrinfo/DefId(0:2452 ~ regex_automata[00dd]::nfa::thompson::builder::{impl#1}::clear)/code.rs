pub fn clear(&mut self) {
        self.pattern_id = None;
        self.states.clear();
        self.start_pattern.clear();
        self.captures.clear();
        self.memory_states = 0;
    }