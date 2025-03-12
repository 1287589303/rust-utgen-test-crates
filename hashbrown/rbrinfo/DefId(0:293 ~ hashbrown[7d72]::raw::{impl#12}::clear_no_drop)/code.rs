fn clear_no_drop(&mut self) {
        if !self.is_empty_singleton() {
            self.ctrl_slice().fill_empty();
        }
        self.items = 0;
        self.growth_left = bucket_mask_to_capacity(self.bucket_mask);
    }