fn get(&self, i: usize) -> Option<Accel> {
        if i >= self.len() {
            return None;
        }
        let offset = ACCEL_TY_SIZE + i * ACCEL_CAP;
        let accel = Accel::from_slice(&self.as_bytes()[offset..])
            .expect("Accels must contain valid accelerators");
        Some(accel)
    }