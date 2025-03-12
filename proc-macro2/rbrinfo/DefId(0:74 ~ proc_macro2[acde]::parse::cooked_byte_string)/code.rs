fn cooked_byte_string(mut input: Cursor) -> Result<Cursor, Reject> {
    let mut bytes = input.bytes().enumerate();
    while let Some((offset, b)) = bytes.next() {
        match b {
            b'"' => {
                let input = input.advance(offset + 1);
                return Ok(literal_suffix(input));
            }
            b'\r' => match bytes.next() {
                Some((_, b'\n')) => {}
                _ => break,
            },
            b'\\' => match bytes.next() {
                Some((_, b'x')) => {
                    backslash_x_byte(&mut bytes)?;
                }
                Some((_, b'n' | b'r' | b't' | b'\\' | b'0' | b'\'' | b'"')) => {}
                Some((newline, b @ (b'\n' | b'\r'))) => {
                    input = input.advance(newline + 1);
                    trailing_backslash(&mut input, b)?;
                    bytes = input.bytes().enumerate();
                }
                _ => break,
            },
            b if b.is_ascii() => {}
            _ => break,
        }
    }
    Err(Reject)
}