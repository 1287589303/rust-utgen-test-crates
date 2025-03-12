pub(crate) fn state_id_error(
        err: StateIDError,
        what: &'static str,
    ) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::StateID { err, what })
    }