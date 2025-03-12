pub fn push(&mut self, interval: I) {
        // TODO: This could be faster. e.g., Push the interval such that
        // it preserves canonicalization.
        self.ranges.push(interval);
        self.canonicalize();
        // We don't know whether the new interval added here is considered
        // case folded, so we conservatively assume that the entire set is
        // no longer case folded if it was previously.
        self.folded = false;
    }