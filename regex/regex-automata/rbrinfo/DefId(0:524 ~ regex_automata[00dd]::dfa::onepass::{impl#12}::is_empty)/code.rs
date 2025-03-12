fn is_empty(self) -> bool {
        self.pattern_id().is_none() && self.epsilons().is_empty()
    }