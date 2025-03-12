fn peek(&self) -> Option<char> {
        if self.is_done() {
            return None;
        }
        self.pattern()[self.pos() + self.char().len_utf8()..].chars().next()
    }