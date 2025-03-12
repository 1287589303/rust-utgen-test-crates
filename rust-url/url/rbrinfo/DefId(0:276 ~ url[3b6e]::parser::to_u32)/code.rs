pub fn to_u32(i: usize) -> ParseResult<u32> {
    if i <= u32::MAX as usize {
        Ok(i as u32)
    } else {
        Err(ParseError::Overflow)
    }
}