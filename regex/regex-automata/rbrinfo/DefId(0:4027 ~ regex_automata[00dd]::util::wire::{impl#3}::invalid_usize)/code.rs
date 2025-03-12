fn invalid_usize(what: &'static str) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::InvalidUsize { what })
    }