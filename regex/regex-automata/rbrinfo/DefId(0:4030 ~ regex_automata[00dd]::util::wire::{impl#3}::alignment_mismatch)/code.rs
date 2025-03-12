fn alignment_mismatch(
        alignment: usize,
        address: usize,
    ) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::AlignmentMismatch {
            alignment,
            address,
        })
    }