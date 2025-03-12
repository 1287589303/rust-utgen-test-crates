fn try_from(index: usize) -> Result<SmallIndex, SmallIndexError> {
        if index > SmallIndex::MAX.as_usize() {
            return Err(SmallIndexError { attempted: index.as_u64() });
        }
        Ok(SmallIndex::new_unchecked(index))
    }