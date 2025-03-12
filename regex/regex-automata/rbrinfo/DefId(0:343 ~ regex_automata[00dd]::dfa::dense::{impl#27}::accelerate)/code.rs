fn accelerate(&self, classes: &ByteClasses) -> Option<Accel> {
        // We just try to add bytes to our accelerator. Once adding fails
        // (because we've added too many bytes), then give up.
        let mut accel = Accel::new();
        for (class, id) in self.transitions() {
            if id == self.id() {
                continue;
            }
            for unit in classes.elements(class) {
                if let Some(byte) = unit.as_u8() {
                    if !accel.add(byte) {
                        return None;
                    }
                }
            }
        }
        if accel.is_empty() {
            None
        } else {
            Some(accel)
        }
    }