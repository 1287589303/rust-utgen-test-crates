fn next_eoi(&self) -> StateID {
        self.next_at(self.ntrans - 1)
    }