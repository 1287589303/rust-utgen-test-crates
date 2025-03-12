pub(crate) fn validate_state_len(
        &self,
        len: usize,
        stride2: usize,
    ) -> Result<(), DeserializeError> {
        // We assume that 'validate' has already passed, so we know that 'max'
        // is truly the max. So all we need to check is that the max state ID
        // is less than the state ID len. The max legal value here is len-1,
        // which occurs when there are no non-special states.
        if (self.max.as_usize() >> stride2) >= len {
            err!("max should not be greater than or equal to state length");
        }
        Ok(())
    }