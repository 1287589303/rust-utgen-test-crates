fn prev_char(ch: char) -> Option<char> {
    // Skip over the surrogate range.
    if ch == '\u{E000}' {
        return Some('\u{D7FF}');
    }
    // OK because subtracting 1 from any valid scalar value other than 0
    // and U+E000 yields a valid scalar value.
    Some(char::from_u32(u32::from(ch).checked_sub(1)?).unwrap())
}