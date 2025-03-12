pub(crate) fn write_version<E: Endian>(
    version: u32,
    dst: &mut [u8],
) -> Result<usize, SerializeError> {
    let nwrite = write_version_len();
    if dst.len() < nwrite {
        return Err(SerializeError::buffer_too_small("version number"));
    }
    E::write_u32(version, dst);
    Ok(nwrite)
}