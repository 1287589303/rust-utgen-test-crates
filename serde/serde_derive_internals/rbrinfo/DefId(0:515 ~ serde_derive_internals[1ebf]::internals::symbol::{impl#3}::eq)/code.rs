fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }