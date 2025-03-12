pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {
        self.0.start_pattern.get(pid.as_usize()).copied()
    }