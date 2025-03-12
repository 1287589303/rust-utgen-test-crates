fn to_owned(&self) -> Transitions<alloc::vec::Vec<u8>> {
        Transitions {
            sparse: self.sparse().to_vec(),
            classes: self.classes.clone(),
            state_len: self.state_len,
            pattern_len: self.pattern_len,
        }
    }