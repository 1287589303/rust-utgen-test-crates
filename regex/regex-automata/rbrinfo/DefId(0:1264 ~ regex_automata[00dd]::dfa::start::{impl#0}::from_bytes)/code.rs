pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(StartKind, usize), DeserializeError> {
        wire::check_slice_len(slice, size_of::<u32>(), "start kind bytes")?;
        let (n, nr) = wire::try_read_u32(slice, "start kind integer")?;
        match n {
            0 => Ok((StartKind::Both, nr)),
            1 => Ok((StartKind::Unanchored, nr)),
            2 => Ok((StartKind::Anchored, nr)),
            _ => Err(DeserializeError::generic("unrecognized start kind")),
        }
    }