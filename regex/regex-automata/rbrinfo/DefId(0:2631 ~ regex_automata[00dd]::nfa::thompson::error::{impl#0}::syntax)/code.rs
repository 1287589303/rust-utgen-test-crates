pub(crate) fn syntax(err: regex_syntax::Error) -> BuildError {
        BuildError { kind: BuildErrorKind::Syntax(err) }
    }