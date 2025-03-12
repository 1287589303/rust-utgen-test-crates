fn as_ref(&self) -> Transitions<&'_ [u8]> {
        Transitions {
            sparse: self.sparse(),
            classes: self.classes.clone(),
            state_len: self.state_len,
            pattern_len: self.pattern_len,
        }
    }