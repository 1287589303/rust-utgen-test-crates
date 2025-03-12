fn digit(&self) -> Option<u32> {
        let byte = *self;
        Some(match byte {
            byte @ b'0'..=b'9' => byte - b'0' + 26,
            byte @ b'A'..=b'Z' => byte - b'A',
            byte @ b'a'..=b'z' => byte - b'a',
            _ => return None,
        } as u32)
    }