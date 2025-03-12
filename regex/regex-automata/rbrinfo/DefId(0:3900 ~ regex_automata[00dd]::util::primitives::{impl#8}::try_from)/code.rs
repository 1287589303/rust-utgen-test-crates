fn try_from(index: u16) -> Result<SmallIndex, SmallIndexError> {
        if u32::from(index) > SmallIndex::MAX.as_u32() {
            return Err(SmallIndexError { attempted: u64::from(index) });
        }
        Ok(SmallIndex::new_unchecked(index.as_usize()))
    }