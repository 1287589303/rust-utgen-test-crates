pub(crate) fn cache(err: CacheError) -> StartError {
        StartError::Cache { err }
    }