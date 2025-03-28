pub(crate) fn decode_last(bytes: &[u8]) -> Option<Result<char, u8>> {
    if bytes.is_empty() {
        return None;
    }
    let mut start = bytes.len() - 1;
    let limit = bytes.len().saturating_sub(4);
    while start > limit && !is_leading_or_invalid_byte(bytes[start]) {
        start -= 1;
    }
    match decode(&bytes[start..]) {
        None => None,
        Some(Ok(ch)) => Some(Ok(ch)),
        Some(Err(_)) => Some(Err(bytes[bytes.len() - 1])),
    }
}