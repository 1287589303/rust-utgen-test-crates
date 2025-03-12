pub fn contains_word(self) -> bool {
        self.contains_word_unicode() || self.contains_word_ascii()
    }