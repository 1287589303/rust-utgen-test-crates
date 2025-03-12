pub(crate) fn read_version(
    slice: &[u8],
    expected_version: u32,
) -> Result<usize, DeserializeError> {
    let (n, nr) = try_read_u32(slice, "version")?;
    assert_eq!(nr, write_version_len());
    if n != expected_version {
        return Err(DeserializeError::version_mismatch(expected_version, n));
    }
    Ok(nr)
}