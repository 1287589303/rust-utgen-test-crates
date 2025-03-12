pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(Flags, usize), DeserializeError> {
        let (bits, nread) = wire::try_read_u32(slice, "flag bitset")?;
        let flags = Flags {
            has_empty: bits & (1 << 0) != 0,
            is_utf8: bits & (1 << 1) != 0,
            is_always_start_anchored: bits & (1 << 2) != 0,
        };
        Ok((flags, nread))
    }