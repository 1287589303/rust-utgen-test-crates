pub(crate) fn try_read_u16(
    slice: &[u8],
    what: &'static str,
) -> Result<(u16, usize), DeserializeError> {
    check_slice_len(slice, size_of::<u16>(), what)?;
    Ok((read_u16(slice), size_of::<u16>()))
}