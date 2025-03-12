fn version_mismatch(expected: u32, found: u32) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::VersionMismatch {
            expected,
            found,
        })
    }