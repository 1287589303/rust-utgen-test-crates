fn parse_hex_digits(&self, digit_len: usize) -> Result<Hir, Error> {
        let mut scratch = String::new();
        for i in 0..digit_len {
            if i > 0 && !self.bump_and_bump_space() {
                return Err(Error::new(ERR_HEX_FIXED_UNEXPECTED_EOF));
            }
            if !is_hex(self.char()) {
                return Err(Error::new(ERR_HEX_FIXED_INVALID_DIGIT));
            }
            scratch.push(self.char());
        }
        // The final bump just moves the parser past the literal, which may
        // be EOF.
        self.bump_and_bump_space();
        match u32::from_str_radix(&scratch, 16).ok().and_then(char::from_u32) {
            None => Err(Error::new(ERR_HEX_FIXED_INVALID)),
            Some(ch) => Ok(self.hir_char(ch)),
        }
    }