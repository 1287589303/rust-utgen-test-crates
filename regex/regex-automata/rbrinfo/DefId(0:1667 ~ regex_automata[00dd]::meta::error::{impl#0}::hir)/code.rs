pub(crate) fn hir(pid: PatternID, err: hir::Error) -> BuildError {
        let err = regex_syntax::Error::from(err);
        BuildError { kind: BuildErrorKind::Syntax { pid, err } }
    }