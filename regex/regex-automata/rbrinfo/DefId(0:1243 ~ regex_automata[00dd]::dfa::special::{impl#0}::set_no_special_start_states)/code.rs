pub(crate) fn set_no_special_start_states(&mut self) {
        use core::cmp::max;
        self.max = max(self.quit_id, max(self.max_match, self.max_accel));
        self.min_start = DEAD;
        self.max_start = DEAD;
    }