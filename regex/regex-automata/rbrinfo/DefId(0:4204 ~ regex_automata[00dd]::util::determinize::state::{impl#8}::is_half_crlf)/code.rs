fn is_half_crlf(&self) -> bool {
        self.0[0] & (1 << 3) > 0
    }