fn trailing_backslash(input: &mut Cursor, mut last: u8) -> Result<(), Reject> {
    let mut whitespace = input.bytes().enumerate();
    loop {
        if last == b'\r' && whitespace.next().map_or(true, |(_, b)| b != b'\n') {
            return Err(Reject);
        }
        match whitespace.next() {
            Some((_, b @ (b' ' | b'\t' | b'\n' | b'\r'))) => {
                last = b;
            }
            Some((offset, _)) => {
                *input = input.advance(offset);
                return Ok(());
            }
            None => return Err(Reject),
        }
    }
}