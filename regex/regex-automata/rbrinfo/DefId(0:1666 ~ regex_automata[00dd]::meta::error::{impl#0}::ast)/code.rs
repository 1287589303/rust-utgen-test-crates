pub(crate) fn ast(pid: PatternID, err: ast::Error) -> BuildError {
        let err = regex_syntax::Error::from(err);
        BuildError { kind: BuildErrorKind::Syntax { pid, err } }
    }