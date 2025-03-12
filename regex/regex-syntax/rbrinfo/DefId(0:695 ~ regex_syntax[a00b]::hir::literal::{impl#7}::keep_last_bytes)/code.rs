pub fn keep_last_bytes(&mut self, len: usize) {
        if len >= self.len() {
            return;
        }
        self.make_inexact();
        self.bytes.drain(..self.len() - len);
    }