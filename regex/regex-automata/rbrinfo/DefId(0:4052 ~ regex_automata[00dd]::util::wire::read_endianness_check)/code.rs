pub(crate) fn read_endianness_check(
    slice: &[u8],
) -> Result<usize, DeserializeError> {
    let (n, nr) = try_read_u32(slice, "endianness check")?;
    assert_eq!(nr, write_endianness_check_len());
    if n != 0xFEFF {
        return Err(DeserializeError::endian_mismatch(0xFEFF, n));
    }
    Ok(nr)
}