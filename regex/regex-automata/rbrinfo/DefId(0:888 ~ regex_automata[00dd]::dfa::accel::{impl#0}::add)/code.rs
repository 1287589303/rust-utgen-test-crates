pub fn add(&mut self, accel: Accel) {
        self.accels.extend_from_slice(&accel.as_accel_tys());
        let len = self.len();
        self.set_len(len + 1);
    }