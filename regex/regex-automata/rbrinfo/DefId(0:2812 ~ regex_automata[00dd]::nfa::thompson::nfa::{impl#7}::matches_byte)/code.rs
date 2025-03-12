pub fn matches_byte(&self, byte: u8) -> Option<StateID> {
        let next = self.transitions[usize::from(byte)];
        if next == StateID::ZERO {
            None
        } else {
            Some(next)
        }
    }