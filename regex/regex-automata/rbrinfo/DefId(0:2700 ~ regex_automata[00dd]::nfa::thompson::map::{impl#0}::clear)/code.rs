pub fn clear(&mut self) {
        if self.map.is_empty() {
            self.map = vec![Utf8BoundedEntry::default(); self.capacity];
        } else {
            self.version = self.version.wrapping_add(1);
            // If we loop back to version 0, then we forcefully clear the
            // entire map. Otherwise, it might be possible to incorrectly
            // match entries used to generate other NFAs.
            if self.version == 0 {
                self.map = vec![Utf8BoundedEntry::default(); self.capacity];
            }
        }
    }