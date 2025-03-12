pub(crate) fn buffer_too_small(what: &'static str) -> SerializeError {
        SerializeError { what }
    }