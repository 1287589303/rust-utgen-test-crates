pub(crate) fn insufficient_cache_capacity(
        minimum: usize,
        given: usize,
    ) -> BuildError {
        BuildError {
            kind: BuildErrorKind::InsufficientCacheCapacity { minimum, given },
        }
    }