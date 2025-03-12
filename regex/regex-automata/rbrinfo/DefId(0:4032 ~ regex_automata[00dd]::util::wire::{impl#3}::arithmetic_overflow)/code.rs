fn arithmetic_overflow(what: &'static str) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::ArithmeticOverflow { what })
    }