fn character(input: Cursor) -> Result<Cursor, Reject> {
    let input = input.parse("'")?;
    let mut chars = input.char_indices();
    let ok = match chars.next().map(|(_, ch)| ch) {
        Some('\\') => match chars.next().map(|(_, ch)| ch) {
            Some('x') => backslash_x_char(&mut chars).is_ok(),
            Some('u') => backslash_u(&mut chars).is_ok(),
            Some('n' | 'r' | 't' | '\\' | '0' | '\'' | '"') => true,
            _ => false,
        },
        ch => ch.is_some(),
    };
    if !ok {
        return Err(Reject);
    }
    let (idx, _) = chars.next().ok_or(Reject)?;
    let input = input.advance(idx).parse("'")?;
    Ok(literal_suffix(input))
}