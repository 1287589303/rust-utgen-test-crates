pub(crate) fn captures(err: captures::GroupInfoError) -> BuildError {
        BuildError { kind: BuildErrorKind::Captures(err) }
    }