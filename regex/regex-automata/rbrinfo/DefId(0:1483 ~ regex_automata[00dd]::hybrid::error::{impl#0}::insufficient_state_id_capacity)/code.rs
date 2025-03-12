pub(crate) fn insufficient_state_id_capacity(
        err: LazyStateIDError,
    ) -> BuildError {
        BuildError {
            kind: BuildErrorKind::InsufficientStateIDCapacity { err },
        }
    }