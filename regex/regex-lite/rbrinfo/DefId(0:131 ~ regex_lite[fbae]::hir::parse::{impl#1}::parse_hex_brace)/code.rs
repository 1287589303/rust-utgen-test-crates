fn parse_hex_brace(&self) -> Result<Hir, Error> {
        let mut scratch = String::new();
        while self.bump_and_bump_space() && self.char() != '}' {
            if !is_hex(self.char()) {
                return Err(Error::new(ERR_HEX_BRACE_INVALID_DIGIT));
            }
            scratch.push(self.char());
        }
        if self.is_done() {
            return Err(Error::new(ERR_HEX_BRACE_UNEXPECTED_EOF));
        }
        assert_eq!(self.char(), '}');
        self.bump_and_bump_space();

        if scratch.is_empty() {
            return Err(Error::new(ERR_HEX_BRACE_EMPTY));
        }
        match u32::from_str_radix(&scratch, 16).ok().and_then(char::from_u32) {
            None => Err(Error::new(ERR_HEX_BRACE_INVALID)),
            Some(ch) => Ok(self.hir_char(ch)),
        }
    }