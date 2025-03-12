pub(crate) fn invalid_capture_index(index: u32) -> BuildError {
        BuildError { kind: BuildErrorKind::InvalidCaptureIndex { index } }
    }