pub fn insert(&mut self, pid: PatternID) -> bool {
        self.try_insert(pid)
            .expect("PatternSet should have sufficient capacity")
    }