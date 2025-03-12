fn byte(input: Cursor) -> Result<Cursor, Reject> {
    let input = input.parse("b'")?;
    let mut bytes = input.bytes().enumerate();
    let ok = match bytes.next().map(|(_, b)| b) {
        Some(b'\\') => match bytes.next().map(|(_, b)| b) {
            Some(b'x') => backslash_x_byte(&mut bytes).is_ok(),
            Some(b'n' | b'r' | b't' | b'\\' | b'0' | b'\'' | b'"') => true,
            _ => false,
        },
        b => b.is_some(),
    };
    if !ok {
        return Err(Reject);
    }
    let (offset, _) = bytes.next().ok_or(Reject)?;
    if !input.chars().as_str().is_char_boundary(offset) {
        return Err(Reject);
    }
    let input = input.advance(offset).parse("'")?;
    Ok(literal_suffix(input))
}