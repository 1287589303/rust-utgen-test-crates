fn delimiter_of_raw_string(input: Cursor) -> PResult<&str> {
    for (i, byte) in input.bytes().enumerate() {
        match byte {
            b'"' => {
                if i > 255 {
                    // https://github.com/rust-lang/rust/pull/95251
                    return Err(Reject);
                }
                return Ok((input.advance(i + 1), &input.rest[..i]));
            }
            b'#' => {}
            _ => break,
        }
    }
    Err(Reject)
}