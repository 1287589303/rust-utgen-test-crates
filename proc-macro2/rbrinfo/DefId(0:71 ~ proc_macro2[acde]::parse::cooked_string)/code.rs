fn cooked_string(mut input: Cursor) -> Result<Cursor, Reject> {
    let mut chars = input.char_indices();

    while let Some((i, ch)) = chars.next() {
        match ch {
            '"' => {
                let input = input.advance(i + 1);
                return Ok(literal_suffix(input));
            }
            '\r' => match chars.next() {
                Some((_, '\n')) => {}
                _ => break,
            },
            '\\' => match chars.next() {
                Some((_, 'x')) => {
                    backslash_x_char(&mut chars)?;
                }
                Some((_, 'n' | 'r' | 't' | '\\' | '\'' | '"' | '0')) => {}
                Some((_, 'u')) => {
                    backslash_u(&mut chars)?;
                }
                Some((newline, ch @ ('\n' | '\r'))) => {
                    input = input.advance(newline + 1);
                    trailing_backslash(&mut input, ch as u8)?;
                    chars = input.char_indices();
                }
                _ => break,
            },
            _ch => {}
        }
    }
    Err(Reject)
}