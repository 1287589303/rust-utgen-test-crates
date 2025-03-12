pub fn finish(mut self) -> Result<(), DecodeError<E>> {
        match (self.buffer_bit_length, self.padding_symbols) {
            (0, 0) => {
                // A multiple of four of alphabet symbols, and nothing else.
            }
            (12, 2) | (12, 0) => {
                // A multiple of four of alphabet symbols, followed by two more symbols,
                // optionally followed by two padding characters (which make a total multiple of four).
                let byte_buffer = [(self.bit_buffer >> 4) as u8];
                (self.write_bytes)(&byte_buffer).map_err(DecodeError::WriteError)?;
            }
            (18, 1) | (18, 0) => {
                // A multiple of four of alphabet symbols, followed by three more symbols,
                // optionally followed by one padding character (which make a total multiple of four).
                let byte_buffer = [(self.bit_buffer >> 10) as u8, (self.bit_buffer >> 2) as u8];
                (self.write_bytes)(&byte_buffer).map_err(DecodeError::WriteError)?;
            }
            (6, _) => return Err(InvalidBase64Details::LoneAlphabetSymbol.into()),
            _ => return Err(InvalidBase64Details::Padding.into()),
        }
        Ok(())
    }