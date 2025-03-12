fn not_one_pass(msg: &'static str) -> BuildError {
        BuildError { kind: BuildErrorKind::NotOnePass { msg } }
    }