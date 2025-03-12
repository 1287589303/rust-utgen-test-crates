fn next_back(&mut self) -> Option<usize> {
        self.0.next_back().map(|pid| pid.as_usize())
    }