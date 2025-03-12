fn raw_c_string(input: Cursor) -> Result<Cursor, Reject> {
    let (input, delimiter) = delimiter_of_raw_string(input)?;
    let mut bytes = input.bytes().enumerate();
    while let Some((i, byte)) = bytes.next() {
        match byte {
            b'"' if input.rest[i + 1..].starts_with(delimiter) => {
                let rest = input.advance(i + 1 + delimiter.len());
                return Ok(literal_suffix(rest));
            }
            b'\r' => match bytes.next() {
                Some((_, b'\n')) => {}
                _ => break,
            },
            b'\0' => break,
            _ => {}
        }
    }
    Err(Reject)
}