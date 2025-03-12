fn char(&self) -> char {
        self.char.get().expect("codepoint, but parser is done")
    }