fn parse_flags(&self) -> Result<Flags, Error> {
        let mut flags = *self.flags.borrow();
        let mut negate = false;
        // Keeps track of whether the previous flag item was a '-'. We use this
        // to detect whether there is a dangling '-', which is invalid.
        let mut last_was_negation = false;
        // A set to keep track of the flags we've seen. Since all flags are
        // ASCII, we only need 128 bytes.
        let mut seen = [false; 128];
        while self.char() != ':' && self.char() != ')' {
            if self.char() == '-' {
                last_was_negation = true;
                if negate {
                    return Err(Error::new(ERR_FLAG_REPEATED_NEGATION));
                }
                negate = true;
            } else {
                last_was_negation = false;
                self.parse_flag(&mut flags, negate)?;
                // OK because every valid flag is ASCII, and we're only here if
                // the flag is valid.
                let flag_byte = u8::try_from(self.char()).unwrap();
                if seen[usize::from(flag_byte)] {
                    return Err(Error::new(ERR_FLAG_DUPLICATE));
                }
                seen[usize::from(flag_byte)] = true;
            }
            if !self.bump() {
                return Err(Error::new(ERR_FLAG_UNEXPECTED_EOF));
            }
        }
        if last_was_negation {
            return Err(Error::new(ERR_FLAG_DANGLING_NEGATION));
        }
        Ok(flags)
    }