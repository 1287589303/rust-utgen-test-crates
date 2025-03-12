pub(crate) fn from_offset(offset: usize) -> RetryFailError {
        RetryFailError { offset }
    }