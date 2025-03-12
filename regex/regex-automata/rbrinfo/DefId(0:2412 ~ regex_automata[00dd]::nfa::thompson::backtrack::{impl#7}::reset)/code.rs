pub fn reset(&mut self, re: &BoundedBacktracker) {
        self.visited.reset(re);
    }