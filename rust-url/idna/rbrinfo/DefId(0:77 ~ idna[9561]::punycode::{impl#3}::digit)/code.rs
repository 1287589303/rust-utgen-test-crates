fn digit(&self) -> Option<u32> {
        let byte = *self;
        Some(match byte {
            byte @ '0'..='9' => u32::from(byte) - u32::from('0') + 26,
            // byte @ 'A'..='Z' => u32::from(byte) - u32::from('A'), // XXX not needed if no public input
            byte @ 'a'..='z' => u32::from(byte) - u32::from('a'),
            _ => return None,
        })
    }