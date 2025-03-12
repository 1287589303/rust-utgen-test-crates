fn set_is_from_word(&mut self) {
        self.0[0] |= 1 << 2;
    }