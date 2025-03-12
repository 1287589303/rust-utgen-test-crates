pub fn feed(&mut self, input: &[u8]) -> Result<(), DecodeError<E>> {
        for &byte in input.iter() {
            let value = BASE64_DECODE_TABLE[byte as usize];
            if value < 0 {
                // A character that’s not part of the alphabet

                // Remove ASCII whitespace
                if matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'\x0C') {
                    continue;
                }

                if byte == b'=' {
                    self.padding_symbols = self.padding_symbols.saturating_add(1);
                    continue;
                }

                return Err(InvalidBase64Details::UnexpectedSymbol(byte).into());
            }
            if self.padding_symbols > 0 {
                return Err(InvalidBase64Details::AlphabetSymbolAfterPadding.into());
            }
            self.bit_buffer <<= 6;
            self.bit_buffer |= value as u32;
            // 18 before incrementing means we’ve just reached 24
            if self.buffer_bit_length < 18 {
                self.buffer_bit_length += 6;
            } else {
                // We’ve accumulated four times 6 bits, which equals three times 8 bits.
                let byte_buffer = [
                    (self.bit_buffer >> 16) as u8,
                    (self.bit_buffer >> 8) as u8,
                    self.bit_buffer as u8,
                ];
                (self.write_bytes)(&byte_buffer).map_err(DecodeError::WriteError)?;
                self.buffer_bit_length = 0;
                // No need to reset bit_buffer,
                // since next time we’re only gonna read relevant bits.
            }
        }
        Ok(())
    }