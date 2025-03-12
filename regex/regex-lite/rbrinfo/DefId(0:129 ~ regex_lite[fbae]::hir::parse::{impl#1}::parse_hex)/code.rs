fn parse_hex(&self) -> Result<Hir, Error> {
        let digit_len = match self.char() {
            'x' => 2,
            'u' => 4,
            'U' => 8,
            unk => unreachable!(
                "invalid start of fixed length hexadecimal number {}",
                unk
            ),
        };
        if !self.bump_and_bump_space() {
            return Err(Error::new(ERR_HEX_UNEXPECTED_EOF));
        }
        if self.char() == '{' {
            self.parse_hex_brace()
        } else {
            self.parse_hex_digits(digit_len)
        }
    }