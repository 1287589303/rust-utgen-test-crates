fn classify_for_punycode(label: &[char]) -> PunycodeClassification {
    let mut iter = label.iter().copied();
    loop {
        if let Some(c) = iter.next() {
            if c.is_ascii() {
                continue;
            }
            if c == '\u{FFFD}' {
                return PunycodeClassification::Error;
            }
            for c in iter {
                if c == '\u{FFFD}' {
                    return PunycodeClassification::Error;
                }
            }
            return PunycodeClassification::Unicode;
        }
        return PunycodeClassification::Ascii;
    }
}