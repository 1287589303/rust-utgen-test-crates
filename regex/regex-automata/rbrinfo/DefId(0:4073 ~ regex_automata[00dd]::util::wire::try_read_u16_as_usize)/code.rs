pub(crate) fn try_read_u16_as_usize(
    slice: &[u8],
    what: &'static str,
) -> Result<(usize, usize), DeserializeError> {
    try_read_u16(slice, what).and_then(|(n, nr)| {
        usize::try_from(n)
            .map(|n| (n, nr))
            .map_err(|_| DeserializeError::invalid_usize(what))
    })
}