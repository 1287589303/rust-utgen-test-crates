fn next(&mut self) -> Option<&'a str> {
        if let Some((&first_byte, remaining)) = self.bytes.split_first() {
            if self.ascii_set.should_percent_encode(first_byte) {
                self.bytes = remaining;
                Some(percent_encode_byte(first_byte))
            } else {
                // The unsafe blocks here are appropriate because the bytes are
                // confirmed as a subset of UTF-8 in should_percent_encode.
                for (i, &byte) in remaining.iter().enumerate() {
                    if self.ascii_set.should_percent_encode(byte) {
                        // 1 for first_byte + i for previous iterations of this loop
                        let (unchanged_slice, remaining) = self.bytes.split_at(1 + i);
                        self.bytes = remaining;
                        return Some(unsafe { str::from_utf8_unchecked(unchanged_slice) });
                    }
                }
                let unchanged_slice = self.bytes;
                self.bytes = &[][..];
                Some(unsafe { str::from_utf8_unchecked(unchanged_slice) })
            }
        } else {
            None
        }
    }