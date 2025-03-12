pub(crate) fn match_len(&self, stride: usize) -> usize {
        if self.matches() {
            (self.max_match.as_usize() - self.min_match.as_usize() + stride)
                / stride
        } else {
            0
        }
    }