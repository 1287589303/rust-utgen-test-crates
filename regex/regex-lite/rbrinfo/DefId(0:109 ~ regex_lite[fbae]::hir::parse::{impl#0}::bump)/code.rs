fn bump(&self) -> bool {
        if self.is_done() {
            return false;
        }
        self.pos.set(self.pos() + self.char().len_utf8());
        self.char.set(self.pattern()[self.pos()..].chars().next());
        self.char.get().is_some()
    }