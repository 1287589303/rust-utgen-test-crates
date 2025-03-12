pub fn to_byte_class(&self) -> Option<ClassBytes> {
        if !self.is_ascii() {
            return None;
        }
        Some(ClassBytes::new(self.ranges().iter().map(|r| {
            // Since we are guaranteed that our codepoint range is ASCII, the
            // 'u8::try_from' calls below are guaranteed to be correct.
            ClassBytesRange {
                start: u8::try_from(r.start).unwrap(),
                end: u8::try_from(r.end).unwrap(),
            }
        })))
    }