pub(crate) fn try_read_u128(
    slice: &[u8],
    what: &'static str,
) -> Result<(u128, usize), DeserializeError> {
    check_slice_len(slice, size_of::<u128>(), what)?;
    Ok((read_u128(slice), size_of::<u128>()))
}