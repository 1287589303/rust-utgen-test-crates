fn try_from(index: u64) -> Result<SmallIndex, SmallIndexError> {
        if index > SmallIndex::MAX.as_u64() {
            return Err(SmallIndexError { attempted: index });
        }
        Ok(SmallIndex::new_unchecked(index.as_usize()))
    }