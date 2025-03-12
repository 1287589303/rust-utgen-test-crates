fn from(err: RetryFailError) -> RetryError {
        RetryError::Fail(err)
    }