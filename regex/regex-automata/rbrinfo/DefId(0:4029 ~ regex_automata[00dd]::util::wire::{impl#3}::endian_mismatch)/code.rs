fn endian_mismatch(expected: u32, found: u32) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::EndianMismatch {
            expected,
            found,
        })
    }