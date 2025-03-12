pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(StartByteMap, usize), DeserializeError> {
        wire::check_slice_len(slice, 256, "start byte map")?;
        let mut map = [Start::NonWordByte; 256];
        for (i, &repr) in slice[..256].iter().enumerate() {
            map[i] = match Start::from_usize(usize::from(repr)) {
                Some(start) => start,
                None => {
                    return Err(DeserializeError::generic(
                        "found invalid starting configuration",
                    ))
                }
            };
        }
        Ok((StartByteMap { map }, 256))
    }