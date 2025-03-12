pub fn is_word_byte(self) -> bool {
        self.as_u8().map_or(false, crate::util::utf8::is_word_byte)
    }