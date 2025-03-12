pub fn to_unicode_class(&self) -> Option<ClassUnicode> {
        if !self.is_ascii() {
            return None;
        }
        Some(ClassUnicode::new(self.ranges().iter().map(|r| {
            // Since we are guaranteed that our byte range is ASCII, the
            // 'char::from' calls below are correct and will not erroneously
            // convert a raw byte value into its corresponding codepoint.
            ClassUnicodeRange {
                start: char::from(r.start),
                end: char::from(r.end),
            }
        })))
    }