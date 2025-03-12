pub(crate) fn shl(
    a: usize,
    b: usize,
    what: &'static str,
) -> Result<usize, DeserializeError> {
    let amount = u32::try_from(b)
        .map_err(|_| DeserializeError::arithmetic_overflow(what))?;
    match a.checked_shl(amount) {
        Some(c) => Ok(c),
        None => Err(DeserializeError::arithmetic_overflow(what)),
    }
}