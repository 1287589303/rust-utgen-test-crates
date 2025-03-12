fn from(m: Match<'h>) -> &'h [u8] {
        m.as_bytes()
    }