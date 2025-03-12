pub fn from_ne_bytes(
        bytes: [u8; 4],
    ) -> Result<SmallIndex, SmallIndexError> {
        let id = u32::from_ne_bytes(bytes);
        if id > SmallIndex::MAX.as_u32() {
            return Err(SmallIndexError { attempted: u64::from(id) });
        }
        Ok(SmallIndex::new_unchecked(id.as_usize()))
    }