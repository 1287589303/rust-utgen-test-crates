fn next_eoi_state(&self, current: StateID) -> StateID {
        let eoi = self.byte_classes().eoi().as_usize();
        let o = current.as_usize() + eoi;
        self.trans()[o]
    }