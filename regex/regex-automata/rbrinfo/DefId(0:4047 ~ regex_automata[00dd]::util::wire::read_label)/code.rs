pub(crate) fn read_label(
    slice: &[u8],
    expected_label: &'static str,
) -> Result<usize, DeserializeError> {
    // Set an upper bound on how many bytes we scan for a NUL. Since no label
    // in this crate is longer than 256 bytes, if we can't find one within that
    // range, then we have corrupted data.
    let first_nul =
        slice[..cmp::min(slice.len(), 256)].iter().position(|&b| b == 0);
    let first_nul = match first_nul {
        Some(first_nul) => first_nul,
        None => {
            return Err(DeserializeError::generic(
                "could not find NUL terminated label \
                 at start of serialized object",
            ));
        }
    };
    let len = first_nul + padding_len(first_nul);
    if slice.len() < len {
        return Err(DeserializeError::generic(
            "could not find properly sized label at start of serialized object"
        ));
    }
    if expected_label.as_bytes() != &slice[..first_nul] {
        return Err(DeserializeError::label_mismatch(expected_label));
    }
    Ok(len)
}