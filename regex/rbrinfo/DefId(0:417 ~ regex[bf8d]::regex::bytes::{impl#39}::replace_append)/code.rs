fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        self.0.replace_append(caps, dst)
    }