pub(crate) fn write_endianness_check<E: Endian>(
    dst: &mut [u8],
) -> Result<usize, SerializeError> {
    let nwrite = write_endianness_check_len();
    if dst.len() < nwrite {
        return Err(SerializeError::buffer_too_small("endianness check"));
    }
    E::write_u32(0xFEFF, dst);
    Ok(nwrite)
}