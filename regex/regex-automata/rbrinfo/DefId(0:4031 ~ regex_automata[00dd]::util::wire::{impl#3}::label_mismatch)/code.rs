fn label_mismatch(expected: &'static str) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::LabelMismatch { expected })
    }