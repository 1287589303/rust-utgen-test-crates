pub fn contains(&self, pid: PatternID) -> bool {
        pid.as_usize() < self.capacity() && self.which[pid]
    }