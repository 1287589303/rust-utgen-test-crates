fn from(merr: MatchError) -> RetryError {
        RetryError::Fail(RetryFailError::from(merr))
    }