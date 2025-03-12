pub fn keep_first_bytes(&mut self, len: usize) {
        if len >= self.len() {
            return;
        }
        self.make_inexact();
        self.bytes.truncate(len);
    }