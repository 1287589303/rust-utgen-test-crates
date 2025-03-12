fn parse_decimal(&self) -> Result<u32, Error> {
        let mut scratch = String::new();
        while !self.is_done() && self.char().is_whitespace() {
            self.bump();
        }
        while !self.is_done() && '0' <= self.char() && self.char() <= '9' {
            scratch.push(self.char());
            self.bump_and_bump_space();
        }
        while !self.is_done() && self.char().is_whitespace() {
            self.bump_and_bump_space();
        }
        let digits = scratch.as_str();
        if digits.is_empty() {
            return Err(Error::new(ERR_DECIMAL_NO_DIGITS));
        }
        match u32::from_str_radix(digits, 10).ok() {
            Some(n) => Ok(n),
            None => Err(Error::new(ERR_DECIMAL_INVALID)),
        }
    }