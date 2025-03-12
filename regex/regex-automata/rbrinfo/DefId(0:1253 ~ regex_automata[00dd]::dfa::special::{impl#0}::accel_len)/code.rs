pub(crate) fn accel_len(&self, stride: usize) -> usize {
        if self.accels() {
            (self.max_accel.as_usize() - self.min_accel.as_usize() + stride)
                / stride
        } else {
            0
        }
    }