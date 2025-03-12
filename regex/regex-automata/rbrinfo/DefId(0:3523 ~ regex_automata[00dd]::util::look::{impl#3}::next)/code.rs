fn next(&mut self) -> Option<Look> {
        if self.set.is_empty() {
            return None;
        }
        // We'll never have more than u8::MAX distinct look-around assertions,
        // so 'bit' will always fit into a u16.
        let bit = u16::try_from(self.set.bits.trailing_zeros()).unwrap();
        let look = Look::from_repr(1 << bit)?;
        self.set = self.set.remove(look);
        Some(look)
    }