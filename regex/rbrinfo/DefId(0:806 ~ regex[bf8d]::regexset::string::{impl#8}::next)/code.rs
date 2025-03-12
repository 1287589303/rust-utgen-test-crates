fn next(&mut self) -> Option<usize> {
        self.0.next().map(|pid| pid.as_usize())
    }