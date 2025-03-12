pub(crate) fn check_alignment<T>(
    slice: &[u8],
) -> Result<(), DeserializeError> {
    let alignment = core::mem::align_of::<T>();
    let address = slice.as_ptr().as_usize();
    if address % alignment == 0 {
        return Ok(());
    }
    Err(DeserializeError::alignment_mismatch(alignment, address))
}