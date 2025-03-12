fn set_is_half_crlf(&mut self) {
        self.0[0] |= 1 << 3;
    }