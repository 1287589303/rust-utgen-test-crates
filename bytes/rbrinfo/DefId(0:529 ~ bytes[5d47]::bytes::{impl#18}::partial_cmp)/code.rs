fn partial_cmp(&self, other: &[u8]) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other)
    }