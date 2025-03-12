pub fn capture_locations(&self) -> CaptureLocations {
        // OK because NFA construction would have failed if this overflowed.
        let len = self.pikevm.nfa().group_len().checked_mul(2).unwrap();
        CaptureLocations(vec![None; len])
    }