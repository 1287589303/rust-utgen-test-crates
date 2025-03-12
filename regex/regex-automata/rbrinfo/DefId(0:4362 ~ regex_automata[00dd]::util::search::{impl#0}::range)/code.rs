pub fn range<R: RangeBounds<usize>>(mut self, range: R) -> Input<'h> {
        self.set_range(range);
        self
    }