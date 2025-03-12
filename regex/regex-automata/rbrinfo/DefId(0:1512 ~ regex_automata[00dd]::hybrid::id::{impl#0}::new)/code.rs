pub(crate) fn new(id: usize) -> Result<LazyStateID, LazyStateIDError> {
        if id > LazyStateID::MAX {
            let attempted = u64::try_from(id).unwrap();
            return Err(LazyStateIDError { attempted });
        }
        Ok(LazyStateID::new_unchecked(id))
    }