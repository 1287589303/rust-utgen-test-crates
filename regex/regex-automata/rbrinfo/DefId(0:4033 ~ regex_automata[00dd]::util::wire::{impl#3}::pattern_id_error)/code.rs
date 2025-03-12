fn pattern_id_error(
        err: PatternIDError,
        what: &'static str,
    ) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::PatternID { err, what })
    }