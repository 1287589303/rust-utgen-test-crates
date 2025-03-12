fn look_have(&self) -> LookSet {
        LookSet::read_repr(&self.0[1..])
    }