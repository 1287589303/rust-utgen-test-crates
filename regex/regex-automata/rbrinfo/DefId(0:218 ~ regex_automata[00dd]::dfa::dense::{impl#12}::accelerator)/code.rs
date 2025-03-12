fn accelerator(&self, id: StateID) -> &[u8] {
        if !self.is_accel_state(id) {
            return &[];
        }
        self.accels.needles(self.accelerator_index(id))
    }