fn accelerator_index(&self, id: StateID) -> usize {
        let min = self.special().min_accel.as_usize();
        // CORRECTNESS: We're allowed to produce an incorrect result or panic,
        // so both the subtraction and the unchecked StateID construction is
        // OK.
        self.to_index(StateID::new_unchecked(id.as_usize() - min))
    }