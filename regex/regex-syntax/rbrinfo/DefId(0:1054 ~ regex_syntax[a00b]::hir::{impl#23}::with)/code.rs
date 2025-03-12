pub fn with(&self, sub: Hir) -> Repetition {
        Repetition {
            min: self.min,
            max: self.max,
            greedy: self.greedy,
            sub: Box::new(sub),
        }
    }