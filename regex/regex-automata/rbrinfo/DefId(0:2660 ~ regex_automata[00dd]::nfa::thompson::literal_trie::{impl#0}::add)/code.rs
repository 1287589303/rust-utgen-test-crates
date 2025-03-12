pub(crate) fn add(&mut self, bytes: &[u8]) -> Result<(), BuildError> {
        let mut prev = StateID::ZERO;
        let mut it = bytes.iter().copied();
        while let Some(b) = if self.rev { it.next_back() } else { it.next() } {
            prev = self.get_or_add_state(prev, b)?;
        }
        self.states[prev].add_match();
        Ok(())
    }