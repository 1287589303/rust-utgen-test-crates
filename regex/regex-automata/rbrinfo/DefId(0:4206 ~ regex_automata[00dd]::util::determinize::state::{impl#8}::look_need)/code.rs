fn look_need(&self) -> LookSet {
        LookSet::read_repr(&self.0[5..])
    }