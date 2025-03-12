pub fn clear(&mut self) {
        self.pid = None;
        for slot in self.slots.iter_mut() {
            *slot = None;
        }
    }