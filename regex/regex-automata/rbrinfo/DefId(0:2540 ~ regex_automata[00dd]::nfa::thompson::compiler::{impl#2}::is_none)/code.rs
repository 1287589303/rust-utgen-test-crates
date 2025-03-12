pub fn is_none(&self) -> bool {
        matches!(*self, WhichCaptures::None)
    }