fn next_char(ch: char) -> Option<char> {
    // Skip over the surrogate range.
    if ch == '\u{D7FF}' {
        return Some('\u{E000}');
    }
    // OK because char::MAX < u32::MAX and we handle U+D7FF above.
    char::from_u32(u32::from(ch).checked_add(1).unwrap())
}